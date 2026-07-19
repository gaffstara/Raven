pub struct RetryManager {
    pub max_attempts: u32,
}

impl RetryManager {
    pub fn new(max_attempts: u32) -> Self {
        Self { max_attempts }
    }

    pub fn should_retry(&self, attempts: u32) -> bool {
        attempts < self.max_attempts
    }
}
