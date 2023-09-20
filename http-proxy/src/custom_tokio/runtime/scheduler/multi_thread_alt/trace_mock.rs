pub(crate) struct TraceStatus {}

impl TraceStatus {
    pub(crate) fn new(_: usize) -> Self {
        Self {}
    }

    pub(crate) fn trace_requested(&self) -> bool {
        false
    }
}
