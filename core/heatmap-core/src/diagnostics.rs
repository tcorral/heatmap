use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct HeatmapDiagnostics {
    pub sampled_out: u64,
}

impl HeatmapDiagnostics {
    pub(crate) fn record_sampled_out(&mut self) {
        self.sampled_out = self.sampled_out.saturating_add(1);
    }
}
