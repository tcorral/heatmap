use crate::db::{self, ElementsFilter};
use crate::validate;
use rusqlite::{Connection, Transaction};
use serde_json::Value;

pub struct IngestResult {
    pub batch_id: String,
    pub accepted_count: i64,
    pub rejected_count: i64,
}

pub fn ingest_batch(
    conn: &Connection,
    batch_id: &str,
    events: &[Value],
) -> Result<IngestResult, rusqlite::Error> {
    if let Some(existing) = db::find_batch(conn, batch_id)? {
        return Ok(IngestResult {
            batch_id: batch_id.to_string(),
            accepted_count: existing.accepted_count,
            rejected_count: existing.rejected_count,
        });
    }

    let mut accepted_count = 0_i64;
    let mut rejected_count = 0_i64;
    let tx = conn.unchecked_transaction()?;
    for event in events {
        if validate::validate_event(event) {
            persist_accepted_event(&tx, batch_id, event)?;
            accepted_count += 1;
        } else {
            rejected_count += 1;
        }
    }
    db::insert_batch(&tx, batch_id, accepted_count, rejected_count)?;
    tx.commit()?;

    Ok(IngestResult {
        batch_id: batch_id.to_string(),
        accepted_count,
        rejected_count,
    })
}

pub fn bin_coord(v: f64) -> i64 {
    let scaled = (v * 100.0).floor() as i64;
    scaled.min(99)
}

fn persist_accepted_event(
    tx: &Transaction<'_>,
    batch_id: &str,
    event: &Value,
) -> Result<(), rusqlite::Error> {
    db::insert_raw_event(tx, batch_id, event)?;
    if is_identified_click(event) {
        db::upsert_element_total(tx, event)?;
    }
    if let Some((bin_x, bin_y)) = click_position_bins(event) {
        db::upsert_click_bin(tx, event, bin_x, bin_y)?;
    }
    Ok(())
}

fn click_position_bins(event: &Value) -> Option<(i64, i64)> {
    if event["event"]["type"].as_str() != Some("click") {
        return None;
    }
    let position = event.get("position")?;
    let x = position.get("x")?.as_f64()?;
    let y = position.get("y")?.as_f64()?;
    Some((bin_coord(x), bin_coord(y)))
}

fn is_identified_click(event: &Value) -> bool {
    let event_type = event["event"]["type"].as_str();
    let element_id = event["event"]
        .get("element_id")
        .and_then(|id| id.as_str());
    event_type == Some("click")
        && element_id.map(|id| !id.is_empty()).unwrap_or(false)
}

pub struct ElementsResponse {
    pub elements: Vec<ElementRow>,
    pub unique_sessions: i64,
}

pub struct ElementRow {
    pub element_id: String,
    pub click_count: i64,
    pub unique_sessions: i64,
}

pub struct HeatmapResponse {
    pub bins: Vec<HeatmapBinRow>,
}

pub struct HeatmapBinRow {
    pub bin_x: i64,
    pub bin_y: i64,
    pub count: i64,
}

pub fn query_heatmap(
    conn: &Connection,
    filter: &db::HeatmapFilter<'_>,
) -> Result<HeatmapResponse, rusqlite::Error> {
    let rows = db::query_click_bins(conn, filter)?;
    let bins = rows
        .into_iter()
        .map(|row| HeatmapBinRow {
            bin_x: row.bin_x,
            bin_y: row.bin_y,
            count: row.count,
        })
        .collect();
    Ok(HeatmapResponse { bins })
}

pub struct ScreenListResponse {
    pub screens: Vec<ScreenListRow>,
}

pub struct ScreenListRow {
    pub platform: String,
    pub id: String,
    pub layout_version: String,
}

pub fn query_screens(conn: &Connection) -> Result<ScreenListResponse, rusqlite::Error> {
    let rows = db::list_distinct_screens(conn)?;
    let screens = rows
        .into_iter()
        .map(|row| ScreenListRow {
            platform: row.platform,
            id: row.id,
            layout_version: row.layout_version,
        })
        .collect();
    Ok(ScreenListResponse { screens })
}

#[cfg(test)]
mod tests {
    use super::bin_coord;

    #[test]
    fn bin_coord_floors_scaled_value_and_caps_at_99() {
        assert_eq!(bin_coord(0.5), 50);
        assert_eq!(bin_coord(1.0), 99);
        assert_eq!(bin_coord(0.999), 99);
    }
}

pub fn query_elements(
    conn: &Connection,
    filter: &ElementsFilter<'_>,
) -> Result<ElementsResponse, rusqlite::Error> {
    let rows = db::query_element_totals(conn, filter)?;
    let unique_sessions = db::count_slice_unique_sessions(conn, filter)?;
    let elements = rows
        .into_iter()
        .map(|row| ElementRow {
            element_id: row.element_id,
            click_count: row.click_count,
            unique_sessions: row.unique_sessions,
        })
        .collect();
    Ok(ElementsResponse {
        elements,
        unique_sessions,
    })
}
