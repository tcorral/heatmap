use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde_json::Value;

pub fn list_raw_events(conn: &Connection) -> SqlResult<Vec<Value>> {
    let mut statement =
        conn.prepare("SELECT event_json FROM raw_events ORDER BY timestamp ASC, id ASC")?;
    let rows = statement.query_map([], |row| {
        let raw: String = row.get(0)?;
        Ok(serde_json::from_str(&raw).unwrap_or(Value::Null))
    })?;
    rows.collect()
}

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

        CREATE TABLE IF NOT EXISTS click_bins (
            platform_type TEXT NOT NULL,
            screen_id TEXT NOT NULL,
            layout_version TEXT NOT NULL,
            app_version TEXT NOT NULL,
            bin_x INTEGER NOT NULL,
            bin_y INTEGER NOT NULL,
            count INTEGER NOT NULL,
            PRIMARY KEY (
                platform_type,
                screen_id,
                layout_version,
                app_version,
                bin_x,
                bin_y
            )
        );

        CREATE TABLE IF NOT EXISTS screen_stills (
            platform_type TEXT NOT NULL,
            screen_id TEXT NOT NULL,
            layout_version TEXT NOT NULL DEFAULT '',
            content_type TEXT NOT NULL,
            image_base64 TEXT NOT NULL,
            PRIMARY KEY (platform_type, screen_id, layout_version)
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
    if filter.from.is_some() || filter.to.is_some() {
        return query_element_totals_from_events(conn, filter);
    }
    query_element_totals_materialized(conn, filter)
}

fn query_element_totals_materialized(
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

fn query_element_totals_from_events(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
) -> SqlResult<Vec<ElementTotalRow>> {
    let layout = filter.layout_version.unwrap_or("");
    let mut statement = conn.prepare(
        "SELECT element_id, COUNT(*) AS click_count FROM raw_events
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
           AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL
           AND element_id != ''
           AND (?5 IS NULL OR timestamp >= ?5)
           AND (?6 IS NULL OR timestamp <= ?6)
         GROUP BY element_id",
    )?;
    let rows = statement.query_map(
        params![
            filter.platform,
            filter.screen,
            layout,
            filter.app_version,
            filter.from,
            filter.to
        ],
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
    conn.query_row(
        "SELECT COUNT(DISTINCT session_id) FROM raw_events
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
           AND app_version = ?4 AND event_type = 'click' AND element_id = ?5
           AND (?6 IS NULL OR timestamp >= ?6)
           AND (?7 IS NULL OR timestamp <= ?7)",
        params![
            filter.platform,
            filter.screen,
            layout,
            filter.app_version,
            element_id,
            filter.from,
            filter.to
        ],
        |row| row.get(0),
    )
}

pub fn upsert_click_bin(
    conn: &Connection,
    event: &Value,
    bin_x: i64,
    bin_y: i64,
) -> SqlResult<()> {
    let platform_type = event["platform"]["type"].as_str().unwrap_or_default();
    let app_version = event["platform"]["app_version"].as_str().unwrap_or_default();
    let screen_id = event["screen"]["id"].as_str().unwrap_or_default();
    let layout_version = event["screen"]
        .get("layout_version")
        .and_then(|version| version.as_str())
        .unwrap_or("");

    conn.execute(
        "INSERT INTO click_bins (
            platform_type, screen_id, layout_version, app_version, bin_x, bin_y, count
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)
        ON CONFLICT(platform_type, screen_id, layout_version, app_version, bin_x, bin_y)
        DO UPDATE SET count = count + 1",
        params![
            platform_type,
            screen_id,
            layout_version,
            app_version,
            bin_x,
            bin_y
        ],
    )?;
    Ok(())
}

pub struct ClickBinRow {
    pub bin_x: i64,
    pub bin_y: i64,
    pub count: i64,
}

pub struct HeatmapFilter<'a> {
    pub platform: &'a str,
    pub screen: &'a str,
    pub layout_version: Option<&'a str>,
    pub app_version: &'a str,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

pub fn query_click_bins(conn: &Connection, filter: &HeatmapFilter<'_>) -> SqlResult<Vec<ClickBinRow>> {
    if filter.from.is_some() || filter.to.is_some() {
        return query_click_bins_from_events(conn, filter);
    }
    query_click_bins_materialized(conn, filter)
}

fn query_click_bins_materialized(
    conn: &Connection,
    filter: &HeatmapFilter<'_>,
) -> SqlResult<Vec<ClickBinRow>> {
    let layout = filter.layout_version.unwrap_or("");
    let mut statement = conn.prepare(
        "SELECT bin_x, bin_y, count FROM click_bins
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3 AND app_version = ?4",
    )?;
    let rows = statement.query_map(
        params![filter.platform, filter.screen, layout, filter.app_version],
        |row| {
            Ok(ClickBinRow {
                bin_x: row.get(0)?,
                bin_y: row.get(1)?,
                count: row.get(2)?,
            })
        },
    )?;
    rows.collect()
}

fn query_click_bins_from_events(
    conn: &Connection,
    filter: &HeatmapFilter<'_>,
) -> SqlResult<Vec<ClickBinRow>> {
    let layout = filter.layout_version.unwrap_or("");
    let sql = "SELECT
            MIN(99, CAST(json_extract(event_json, '$.position.x') * 100 AS INTEGER)) AS bin_x,
            MIN(99, CAST(json_extract(event_json, '$.position.y') * 100 AS INTEGER)) AS bin_y,
            COUNT(*) AS count
         FROM raw_events
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
           AND app_version = ?4 AND event_type = 'click'
           AND json_extract(event_json, '$.position.x') IS NOT NULL
           AND json_extract(event_json, '$.position.y') IS NOT NULL
           AND (?5 IS NULL OR timestamp >= ?5)
           AND (?6 IS NULL OR timestamp <= ?6)
         GROUP BY bin_x, bin_y";
    let mut statement = conn.prepare(sql)?;
    let rows = statement.query_map(
        params![
            filter.platform,
            filter.screen,
            layout,
            filter.app_version,
            filter.from,
            filter.to
        ],
        |row| {
            Ok(ClickBinRow {
                bin_x: row.get(0)?,
                bin_y: row.get(1)?,
                count: row.get(2)?,
            })
        },
    )?;
    rows.collect()
}

pub struct StillRecord {
    pub content_type: String,
    pub image_base64: String,
}

pub fn upsert_still(
    conn: &Connection,
    platform: &str,
    screen: &str,
    layout_version: &str,
    content_type: &str,
    image_base64: &str,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO screen_stills (
            platform_type, screen_id, layout_version, content_type, image_base64
        ) VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(platform_type, screen_id, layout_version)
        DO UPDATE SET content_type = excluded.content_type,
                      image_base64 = excluded.image_base64",
        params![platform, screen, layout_version, content_type, image_base64],
    )?;
    Ok(())
}

pub fn find_still(
    conn: &Connection,
    platform: &str,
    screen: &str,
    layout_version: &str,
) -> SqlResult<Option<StillRecord>> {
    conn.query_row(
        "SELECT content_type, image_base64 FROM screen_stills
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3",
        params![platform, screen, layout_version],
        |row| {
            Ok(StillRecord {
                content_type: row.get(0)?,
                image_base64: row.get(1)?,
            })
        },
    )
    .optional()
}

pub struct ScreenRow {
    pub platform: String,
    pub id: String,
    pub layout_version: String,
}

pub fn list_distinct_screens(conn: &Connection) -> SqlResult<Vec<ScreenRow>> {
    let mut statement = conn.prepare(
        "SELECT DISTINCT platform_type, screen_id, layout_version FROM raw_events
         WHERE screen_id != '' ORDER BY platform_type, screen_id, layout_version",
    )?;
    let rows = statement.query_map([], |row| {
        Ok(ScreenRow {
            platform: row.get(0)?,
            id: row.get(1)?,
            layout_version: row.get(2)?,
        })
    })?;
    rows.collect()
}

pub fn count_slice_unique_sessions(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
) -> SqlResult<i64> {
    let layout = filter.layout_version.unwrap_or("");
    conn.query_row(
        "SELECT COUNT(DISTINCT session_id) FROM raw_events
         WHERE platform_type = ?1 AND screen_id = ?2 AND layout_version = ?3
           AND app_version = ?4 AND event_type = 'click' AND element_id IS NOT NULL
           AND (?5 IS NULL OR timestamp >= ?5)
           AND (?6 IS NULL OR timestamp <= ?6)",
        params![
            filter.platform,
            filter.screen,
            layout,
            filter.app_version,
            filter.from,
            filter.to
        ],
        |row| row.get(0),
    )
}
