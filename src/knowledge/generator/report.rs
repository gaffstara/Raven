use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct GeneratorReport {
    pub files_processed: usize,
    pub chunks_generated: usize,
    pub duplicates_removed: usize,
    pub knowledge_stored: usize,
    pub duration: Option<Duration>,
    pub errors: usize,
}

impl GeneratorReport {
    pub fn summary(&self) -> String {
        format!(
            "files={} chunks={} duplicates={} stored={} errors={} duration={:?}",
            self.files_processed,
            self.chunks_generated,
            self.duplicates_removed,
            self.knowledge_stored,
            self.errors,
            self.duration
        )
    }
}
