use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};

pub struct Cleaner;

impl Cleaner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clean(&self, input: &str) -> KnowledgeResult<String> {
        // Normalize line endings, remove excessive blank lines and control chars
        let mut out = input.replace('\r', "");
        // Replace multiple blank lines with single
        let mut prev_blank = false;
        let mut lines = Vec::new();
        for line in out.lines() {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                if prev_blank {
                    continue;
                }
                prev_blank = true;
                lines.push(String::new());
            } else {
                prev_blank = false;
                // remove unusual invisible chars
                let filtered: String = trimmed.chars().filter(|c| !c.is_control() || *c=='\n' || *c=='\t').collect();
                lines.push(filtered);
            }
        }
        Ok(lines.join("\n"))
    }
}
