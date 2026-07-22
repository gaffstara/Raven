use crate::knowledge::errors::{KnowledgeError, KnowledgeResult};
use std::collections::HashMap;

/// Simple markdown-ish parser that extracts headings and code blocks.
pub struct SimpleParser;

impl SimpleParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_title(&self, content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                // take first heading as title
                let title = trimmed.trim_start_matches('#').trim().to_string();
                if !title.is_empty() {
                    return Some(title);
                }
            }
        }
        None
    }

    pub fn extract_headings(&self, content: &str) -> Vec<String> {
        let mut hs = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                hs.push(t.trim_start_matches('#').trim().to_string());
            }
        }
        hs
    }

    pub fn extract_code_blocks(&self, content: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut current = Vec::new();
        for line in content.lines() {
            if line.trim_start().starts_with("```") {
                if in_block {
                    blocks.push(current.join("\n"));
                    current.clear();
                    in_block = false;
                } else {
                    in_block = true;
                }
                continue;
            }
            if in_block {
                current.push(line.to_string());
            }
        }
        if in_block && !current.is_empty() {
            blocks.push(current.join("\n"));
        }
        blocks
    }

    pub fn extract_sections(&self, content: &str) -> HashMap<String, String> {
        let mut sections = HashMap::new();
        let mut current_title = String::new();
        let mut current_body = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                if !current_title.is_empty() {
                    sections.insert(current_title.clone(), current_body.join("\n"));
                    current_body.clear();
                }
                current_title = t.trim_start_matches('#').trim().to_string();
            } else {
                current_body.push(line.to_string());
            }
        }
        if !current_title.is_empty() {
            sections.insert(current_title, current_body.join("\n"));
        }
        sections
    }
}
