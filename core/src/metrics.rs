pub trait MetricsSink: Send + Sync {
    fn incr(&self, key: &str);
    fn timing_ms(&self, key: &str, value: u64);
}

#[derive(Clone, Copy, Debug)]
pub struct NoopMetrics;

impl MetricsSink for NoopMetrics {
    fn incr(&self, _key: &str) {}
    fn timing_ms(&self, _key: &str, _value: u64) {}
}
