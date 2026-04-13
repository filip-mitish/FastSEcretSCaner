use tabled::Tabled;
use colored::*;
use crate::scanner::Detection;

#[derive(Tabled)]
pub struct ScanResultRow {
    #[tabled(rename = "Severity")]
    pub severity: String,
    #[tabled(rename = "File")]
    pub file: String,
    #[tabled(rename = "Line")]
    pub line: usize,
    #[tabled(rename = "Pattern")]
    pub pattern: String,
    #[tabled(rename = "Confidence")]
    pub confidence: String,
}

impl ScanResultRow {
    pub fn from_detection(det: &Detection) -> Self {
        let severity = if det.confidence > 0.8 {
            "CRITICAL".red().bold().to_string()
        } else if det.confidence > 0.5 {
            "HIGH".yellow().bold().to_string()
        } else {
            "INFO".blue().to_string()
        };

        Self {
            severity,
            file: det.file_path.clone(),
            line: det.line_number,
            pattern: det.pattern_name.clone(),
            confidence: format!("{:.0}%", det.confidence * 100.0),
        }
    }
}
