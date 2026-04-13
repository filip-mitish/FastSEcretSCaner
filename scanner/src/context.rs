use std::path::Path;

pub struct HeuristicEngine;

impl HeuristicEngine {
    pub fn assess_confidence(path: &Path, value: &str, pattern_name: &str) -> f64 {
        let mut score: f64 = 0.5;

        let path_str = path.to_string_lossy().to_lowercase();
        let value_lower = value.to_lowercase();

        if path_str.contains("test") || path_str.contains("mock") || path_str.contains("spec") {
            score -= 0.3;
        }
        if path_str.contains("node_modules") || path_str.contains("vendor") {
            score -= 0.1;
        }
        if path_str.contains("example") || path_str.contains("sample") {
            score -= 0.2;
        }

        if value_lower.contains("dummy") || value_lower.contains("fake") || value_lower.contains("example") {
            score -= 0.4;
        }
        
        if pattern_name == "AWS Access Key ID" && value_lower.contains("example") {
            score = 0.0;
        }

        let entropy = calculate_entropy(value);
        if entropy > 4.5 {
            score += 0.3;
        } else if entropy < 3.0 {
            score -= 0.2;
        }

        score.clamp(0.0, 1.0)
    }
}

fn calculate_entropy(text: &str) -> f64 {
    use std::collections::HashMap;
    if text.is_empty() { return 0.0; }
    let mut frequencies = HashMap::new();
    let len = text.len() as f64;
    for c in text.chars() { *frequencies.entry(c).or_insert(0) += 1; }
    let mut entropy = 0.0;
    for &count in frequencies.values() {
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }
    entropy
}
