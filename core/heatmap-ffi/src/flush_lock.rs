use std::sync::{Mutex, MutexGuard};

use heatmap_core::FlushError;

use crate::public_api::PublicHeatmapSdk;

pub(crate) fn run_prepared_flush(
    state: &Mutex<PublicHeatmapSdk>,
    require_auto: bool,
) -> Result<(), FlushError> {
    let prepared = {
        let mut guard = lock_sdk(state);
        guard.prepare_flush(require_auto)
    };
    let Some(prepared) = prepared else {
        return Ok(());
    };
    let result = prepared.send();
    let mut guard = lock_sdk(state);
    guard.complete_prepared_flush(prepared, result)
}

fn lock_sdk(state: &Mutex<PublicHeatmapSdk>) -> MutexGuard<'_, PublicHeatmapSdk> {
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
