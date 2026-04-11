mod scanner;
mod hook;
mod context;
mod verifier;
mod ui;
mod report;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use tabled::Table;

use crate::scanner::{Scanner, Detection};
use crate::hook::HookManager;
use crate::verifier::Verifier;
use crate::report::ScanResultRow;

#[derive(Parser)]
#[command(name = "fsesc")]
#[command(about = "FSESC: Ultra-fast Secret Scanner with Verification", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        path: Option<PathBuf>,
        
        #[arg(short, long)]
        verify: bool,

        #[arg(short, long)]
        all: bool,

        #[arg(short, long)]
        json: bool,
    },
    InitHook,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path, verify, all, json } => {
            let scan_path = path.clone().unwrap_or(std::env::current_dir()?);
            run_scan(scan_path, *verify, *all, *json).await?;
        }
        Commands::InitHook => {
            HookManager::install()?;
            println!("{} Pre-commit hook installed successfully.", "✔".green());
        }
    }

    Ok(())
}

async fn run_scan(path: PathBuf, verify: bool, show_all: bool, json_output: bool) -> Result<()> {
    let start = Instant::now();
    let scanner = Scanner::new();
    let file_count = AtomicUsize::new(0);

    if !json_output {
        ui::print_banner();
        println!("{} Indexing files in {}...", "📂".cyan(), path.display());
    }

    let walker = WalkBuilder::new(&path)
        .hidden(true)
        .git_ignore(true)
        .build();

    let files: Vec<PathBuf> = walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    if !json_output {
        println!("{} Found {} files to scan.", "🔍".blue(), files.len());
    }

    let pb = if !json_output {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };

    let mut all_detections: Vec<Detection> = files
        .par_iter()
        .flat_map(|file_path| {
            if let Some(ref p) = pb { p.inc(1); }
            file_count.fetch_add(1, Ordering::Relaxed);
            match scanner.scan_file(file_path) {
                Ok(detections) => {
                    if show_all {
                        detections
                    } else {
                        detections.into_iter().filter(|d| d.confidence > 0.4).collect()
                    }
                }
                Err(_) => vec![],
            }
        })
        .collect();

    if let Some(ref p) = pb { p.finish_with_message("Scan Complete"); }

    if verify && !all_detections.is_empty() {
        if !json_output {
            println!("{} Verifying {} potential secrets...", "🌐".magenta(), all_detections.len());
        }
        let verifier = Verifier::new();
        scanner.verify_detections(&mut all_detections, &verifier).await;
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(&all_detections)?);
    } else {
        if all_detections.is_empty() {
            println!("\n{} {}", "✨".green(), "No secrets found. Your repository is safe!".green().bold());
        } else {
            println!("\n{}", "⚠️  DETECTIONS FOUND".red().bold());
            let rows: Vec<ScanResultRow> = all_detections.iter().map(ScanResultRow::from_detection).collect();
            let table = Table::new(rows).to_string();
            println!("{}", table);
        }

        let duration = start.elapsed();
        println!("\n{}", "─".repeat(50).dimmed());
        println!(
            "{} Finished in {:.2?} | {} Files Scanned | {} Secrets Found",
            "📊".yellow(),
            duration,
            file_count.load(Ordering::SeqCst),
            all_detections.len()
        );
    }

    if !all_detections.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
