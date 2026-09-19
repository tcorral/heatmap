use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde_json::Value;

pub fn open_in_memory() -> SqlResult<Connection> {
    let conn = Connection::open_in_memory()?;
    init_schema(&conn)?;
    Ok(conn)
}

pub fn init_schema(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS batches (
            batch_id TEXT PRIMARY KEY,
            accepted_count INTEGER NOT NULL,
            rejected_count INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS raw_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            batch_id TEXT NOT NULL,
            session_id TEXT NOT NULL,
            timestamp REAL NOT NULL,
            platform_type TEXT NOT NULL,
            app_version TEXT NOT NULL,
            screen_id TEXT NOT NULL DEFAULT '',
            layout_version TEXT NOT NULL DEFAULT '',
            event_type TEXT NOT NULL,
            element_id TEXT,
            event_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS element_totals (
            platform_type TEXT NOT NULL,
            screen_id TEXT NOT NULL,
            layout_version TEXT NOT NULL,
            app_version TEXT NOT NULL,
            element_id TEXT NOT NULL,
            click_count INTEGER NOT NULL,
            PRIMARY KEY (
                platform_type,
                screen_id,
                layout_version,
                app_version,
                element_id
            )
        );
        ",
    )
}

pub struct BatchRecord {
    pub accepted_count: i64,
    pub rejected_count: i64,
}

pub fn find_batch(conn: &Connection, batch_id: &str) -> SqlResult<Option<BatchRecord>> {
    conn.query_row(
        "SELECT accepted_count, rejected_count FROM batches WHERE batch_id = ?1",
        [batch_id],
        |row| {
            Ok(BatchRecord {
                accepted_count: row.get(0)?,
                rejected_count: row.get(1)?,
            })
        },
    )
    .optional()
}

pub fn insert_batch(
    conn: &Connection,
    batch_id: &str,
    accepted_count: i64,
    rejected_count: i64,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO batches (batch_id, accepted_count, rejected_count) VALUES (?1, ?2, ?3)",
        params![batch_id, accepted_count, rejected_count],
    )?;
    Ok(())
}

pub fn insert_raw_event(conn: &Connection, batch_id: &str, event: &Value) -> SqlResult<()> {
    let session_id = event["session_id"].as_str().unwrap_or_default();
    let timestamp = event["timestamp"].as_f64().unwrap_or(0.0);
    let platform_type = event["platform"]["type"].as_str().unwrap_or_default();
    let app_version = event["platform"]["app_version"].as_str().unwrap_or_default();
    let screen_id = event
        .get("screen")
        .and_then(|screen| screen.get("id"))
        .and_then(|id| id.as_str())
        .unwrap_or("");
    let layout_version = event
        .get("screen")
        .and_then(|screen| screen.get("layout_version"))
        .and_then(|version| version.as_str())
        .unwrap_or("");
    let event_type = event["event"]["type"].as_str().unwrap_or_default();
    let element_id = event["event"]
        .get("element_id")
        .and_then(|id| id.as_str());
    let event_json = event.to_string();

    conn.execute(
        "INSERT INTO raw_events (
            batch_id, session_id, timestamp, platform_type, app_version,
            screen_id, layout_version, event_type, element_id, event_json
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            batch_id,
            session_id,
            timestamp,
            platform_type,
            app_version,
            screen_id,
            layout_version,
            event_type,
            element_id,
            event_json
        ],
    )?;
    Ok(())
}

pub fn upsert_element_total(conn: &Connection, event: &Value) -> SqlResult<()> {
    let platform_type = event["platform"]["type"].as_str().unwrap_or_default();
    let app_version = event["platform"]["app_version"].as_str().unwrap_or_default();
    let screen_id = event["screen"]["id"].as_str().unwrap_or_default();
    let layout_version = event["screen"]
        .get("layout_version")
        .and_then(|version| version.as_str())
        .unwrap_or("");
    let element_id = event["event"]["element_id"]
        .as_str()
        .unwrap_or_default();

    conn.execute(
        "INSERT INTO element_totals (
            platform_type, screen_id, layout_version, app_version, element_id, click_count
        ) VALUES (?1, ?2, ?3, ?4, ?5, 1)
        ON CONFLICT(platform_type, screen_id, layout_version, app_version, element_id)
        DO UPDATE SET click_count = click_count + 1",
        params![
            platform_type,
            screen_id,
            layout_version,
            app_version,
            element_id
        ],
    )?;
    Ok(())
}

pub struct ElementTotalRow {
    pub element_id: String,
    pub click_count: i64,
    pub unique_sessions: i64,
}

pub struct ElementsFilter<'a> {
    pub platform: &'a str,
    pub screen: &'a str,
    pub layout_version: Option<&'a str>,
    pub app_version: &'a str,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

pub fn query_element_totals(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
) -> SqlResult<Vec<ElementTotalRow>> {
    let layout = filter.layout_version.unwrap_or("");
    let mut statement = conn.prepare(
        "SELECT element_id, click_count FROM element_totals
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3 AND app_version = ?4",
    )?;
    let rows = statement.query_map(
        params![filter.platform, filter.screen, layout, filter.app_version],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
    )?;

    let mut results = Vec::new();
    for row in rows {
        let (element_id, click_count) = row?;
        let unique_sessions = count_unique_sessions(conn, filter, &element_id)?;
        results.push(ElementTotalRow {
            element_id,
            click_count,
            unique_sessions,
        });
    }
    Ok(results)
}

fn count_unique_sessions(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
    element_id: &str,
) -> SqlResult<i64> {
    let layout = filter.layout_version.unwrap_or("");
    match (filter.from, filter.to) {
        (Some(from), Some(to)) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id = ?5
             AND timestamp >= ?6 AND timestamp <= ?7",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                element_id,
                from,
                to
            ],
            |row| row.get(0),
        ),
        (Some(from), None) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id = ?5
             AND timestamp >= ?6",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                element_id,
                from
            ],
            |row| row.get(0),
        ),
        (None, Some(to)) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id = ?5
             AND timestamp <= ?6",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                element_id,
                to
            ],
            |row| row.get(0),
        ),
        (None, None) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id = ?5",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                element_id
            ],
            |row| row.get(0),
        ),
    }
}

pub fn count_slice_unique_sessions(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
) -> SqlResult<i64> {
    let layout = filter.layout_version.unwrap_or("");
    match (filter.from, filter.to) {
        (Some(from), Some(to)) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL
             AND timestamp >= ?5 AND timestamp <= ?6",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                from,
                to
            ],
            |row| row.get(0),
        ),
        (Some(from), None) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL
             AND timestamp >= ?5",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                from
            ],
            |row| row.get(0),
        ),
        (None, Some(to)) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL
             AND timestamp <= ?5",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version,
                to
            ],
            |row| row.get(0),
        ),
        (None, None) => conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM raw_events
             WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
             AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL",
            params![
                filter.platform,
                filter.screen,
                layout,
                filter.app_version
            ],
            |row| row.get(0),
        ),
    }
}
