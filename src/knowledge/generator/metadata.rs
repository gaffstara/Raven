use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use crate::knowledge::loader::parse_frontmatter;
use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;

/// Extract metadata from frontmatter and content heuristics.
pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(&self, path: &Path, raw: &str) -> KnowledgeResult<HashMap<String, String>> {
        // prefer frontmatter when present
        match parse_frontmatter(raw) {
            Ok((fm, _body)) => {
                if !fm.is_empty() {
                    return Ok(fm);
                }
            }
            Err(_) => {
                // if no frontmatter, fall through to heuristics
            }
        }

        let mut map = HashMap::new();
        let now = Utc::now().to_rfc3339();
        let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("document");
        map.insert("title".to_string(), file_name.to_string());
        map.insert("language".to_string(), "unknown".to_string());
        map.insert("category".to_string(), "general".to_string());
        map.insert("version".to_string(), "1.0".to_string());
        map.insert("difficulty".to_string(), "intermediate".to_string());
        map.insert("source".to_string(), path.display().to_string());
        map.insert("last_updated".to_string(), now.clone());
        map.insert("created_at".to_string(), now);
        Ok(map)
    }
}
