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

fn persist_accepted_event(
    tx: &Transaction<'_>,
    batch_id: &str,
    event: &Value,
) -> Result<(), rusqlite::Error> {
    db::insert_raw_event(tx, batch_id, event)?;
    if is_identified_click(event) {
        db::upsert_element_total(tx, event)?;
    }
    Ok(())
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
