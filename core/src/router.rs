use crate::metrics::MetricsSink;
use crate::types::NormalizedEvent;

pub struct Router<M: MetricsSink> {
    metrics: M,
}

impl<M: MetricsSink> Router<M> {
    pub fn new(metrics: M) -> Self {
        Self { metrics }
    }

    pub fn route(&self, events: Vec<NormalizedEvent>) {
        // Placeholder fan-out; integrate with user channels in higher layers.
        let _ = events.len();
        self.metrics.incr("router.events");
    }
}
