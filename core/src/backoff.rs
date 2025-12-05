#[derive(Clone, Debug)]
pub struct BackoffPolicy {
    pub max_retries: u32,
    pub base_millis: u64,
    pub max_millis: u64,
}

impl Default for BackoffPolicy {
    fn default() -> Self {
        Self {
            max_retries: 10,
            base_millis: 500,
            max_millis: 30_000,
        }
    }
}

impl BackoffPolicy {
    pub fn delay_ms(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return self.base_millis;
        }
        let capped_attempt = attempt.min(self.max_retries);
        let delay = self.base_millis.saturating_mul(2u64.saturating_pow(capped_attempt));
        delay.min(self.max_millis)
    }
}
