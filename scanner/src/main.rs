mod scanner;
mod context;
mod ui;
mod report;
mod hook;

use clap::{Parser, Subcommand};
use scanner::Scanner;
use ui::{print_banner, format_count};
use report::ScanResultRow;
use tabled::Table;
use std::time::Instant;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fsesc")]
#[command(about = "Fast SEcret SCanner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        path: String,
        #[arg(short, long)]
        all: bool,
    },
    InstallHook,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { path, all } => {
            print_banner();
            let start = Instant::now();
            let scanner = Scanner::new(all);
            
            let path_buf = PathBuf::from(&path);
            let detections = scanner.scan_path(path_buf);
            
            if !detections.is_empty() {
                let rows: Vec<ScanResultRow> = detections.iter().map(ScanResultRow::from_detection).collect();
                println!("{}", Table::new(rows).to_string());
            }

            let duration = start.elapsed();
            println!("\nSTATS:");
            println!("Duration: {:?}", duration);
            println!("Findings: {}", format_count(detections.len()));
            
            if !detections.is_empty() {
                std::process::exit(1);
            }
        }
        Commands::InstallHook => {
            match hook::install_git_hook() {
                Ok(_) => println!("SUCCESS: Pre-commit hook installed."),
                Err(e) => eprintln!("ERROR: Failed to install hook: {}", e),
            }
        }
    }
}
