use anyhow::Result;
use memmap2::Mmap;
use regex::{Regex, RegexSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use ignore::WalkBuilder;
use crate::context::HeuristicEngine;
use crate::verifier::{Verifier, VerificationResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub file_path: String,
    pub line_number: usize,
    pub pattern_name: String,
    pub secret_value: String,
    pub entropy: f64,
    pub confidence: f64,
    pub verification: Option<VerificationResult>,
}

pub struct Scanner {
    patterns: RegexSet,
    extractors: Vec<(String, Regex)>,
    all_files: bool,
}

impl Scanner {
    pub fn new(all_files: bool) -> Self {
        let rules = vec![
            ("AWS Access Key ID", r#"(?:A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}"#),
            ("AWS Secret Access Key", r#"(?i)aws_(?:secret|key|access_key)\s*[:=]\s*['"]?([a-zA-Z0-9/+=]{40})['"]?"#),
            ("GitHub Personal Access Token", r#"ghp_[a-zA-Z0-9]{36}"#),
            ("Google API Key", r#"AIza[0-9A-Za-z-_]{35}"#),
            ("Slack Token", r#"xox[baprs]-[0-9a-zA-Z]{10,48}"#),
            ("Stripe API Key", r#"(?:r|s)k_(?:live|test)_[0-9a-zA-Z]{24}"#),
            ("Generic Password/Secret", r#"(?i)(?:password|passwd|secret|passphrase|token|auth)\s*[:=]\s*['"]?([a-zA-Z0-9!@#$%^&*()_\-+=]{12,64})['"]?"#),
            ("Private Key", r#"-----BEGIN (?:RSA|OPENSSH|DSA|EC|PGP) PRIVATE KEY-----"#),
        ];

        let regex_set = RegexSet::new(rules.iter().map(|(_, r)| *r)).expect("Failed to compile RegexSet");
        let extractors = rules
            .into_iter()
            .map(|(name, r)| (name.to_string(), Regex::new(r).expect("Failed to compile individual Regex")))
            .collect();

        Self {
            patterns: regex_set,
            extractors,
            all_files,
        }
    }

    pub async fn scan_path(&self, root: PathBuf, verify: bool) -> Vec<Detection> {
        let verifier = Verifier::new();
        let mut results = Vec::new();
        
        let walker = WalkBuilder::new(root)
            .hidden(!self.all_files)
            .git_ignore(!self.all_files)
            .build();

        for entry in walker.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                if let Ok(mut detections) = self.scan_file(entry.path()) {
                    if verify {
                        self.verify_detections(&mut detections, &verifier).await;
                    }
                    results.extend(detections);
                }
            }
        }
        results
    }

    pub fn scan_file(&self, path: &Path) -> Result<Vec<Detection>> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        if metadata.len() == 0 || metadata.is_dir() { return Ok(vec![]); }

        let mmap = unsafe { Mmap::map(&file)? };
        let content = std::str::from_utf8(&mmap).unwrap_or("");

        let mut detections = Vec::new();
        let matches: Vec<_> = self.patterns.matches(content).into_iter().collect();

        if !matches.is_empty() {
            for idx in matches {
                let (name, regex) = &self.extractors[idx];
                for cap in regex.captures_iter(content) {
                    let full_match = cap.get(0).unwrap();
                    let value = cap.get(1).map(|m| m.as_str()).unwrap_or(full_match.as_str());
                    
                    let entropy = calculate_entropy(value);
                    let confidence = HeuristicEngine::assess_confidence(path, value, name);

                    if confidence < 0.2 { continue; }

                    let line_number = content[..full_match.start()].lines().count() + 1;
                    detections.push(Detection {
                        file_path: path.to_string_lossy().to_string(),
                        line_number,
                        pattern_name: name.clone(),
                        secret_value: value.to_string(),
                        entropy,
                        confidence,
                        verification: None,
                    });
                }
            }
        }

        Ok(detections)
    }

    async fn verify_detections(&self, detections: &mut Vec<Detection>, verifier: &Verifier) {
        for det in detections {
            let res = verifier.verify(&det.pattern_name, &det.secret_value).await;
            det.verification = Some(res);
        }
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
