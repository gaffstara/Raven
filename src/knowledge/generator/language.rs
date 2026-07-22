use std::path::Path;

/// Heuristic language detection based on file extension and content clues.
#[derive(Debug, Clone)]
pub struct LanguageDetector;

impl LanguageDetector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn detect(&self, path: &Path, content: &str) -> String {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext.to_lowercase().as_str() {
                "rs" => return "rust".to_string(),
                "go" => return "go".to_string(),
                "py" => return "python".to_string(),
                "java" => return "java".to_string(),
                "js" | "mjs" | "cjs" => return "javascript".to_string(),
                "md" => return "markdown".to_string(),
                "toml" => return "toml".to_string(),
                "yaml" | "yml" => return "yaml".to_string(),
                "json" => return "json".to_string(),
                _ => {}
            }
        }

        let lower = content.to_lowercase();
        if lower.contains("fn main()") || lower.contains("println!") {
            return "rust".to_string();
        }
        if lower.contains("package main") || lower.contains("goroutine") || lower.contains("go func") {
            return "go".to_string();
        }
        if lower.contains("def ") || lower.contains("import sys") {
            return "python".to_string();
        }
        if lower.contains("class ") && lower.contains("public static void main") {
            return "java".to_string();
        }
        if lower.contains("function") || lower.contains("console.log") {
            return "javascript".to_string();
        }

        "text".to_string()
    }
}
