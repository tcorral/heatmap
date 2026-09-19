use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct HeatmapDiagnostics {
    pub sampled_out: u64,
    pub queue_dropped: u64,
}

impl HeatmapDiagnostics {
    pub(crate) fn record_sampled_out(&mut self) {
        self.sampled_out = self.sampled_out.saturating_add(1);
    }

    pub(crate) fn record_queue_dropped(&mut self, count: u64) {
        self.queue_dropped = self.queue_dropped.saturating_add(count);
    }
}
