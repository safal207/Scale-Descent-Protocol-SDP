use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use sdp_core::{collapse_hypotheses, DescentRun};

#[derive(Debug, Parser)]
#[command(name = "sdp")]
#[command(about = "Scale Descent Protocol inspector")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Inspect { file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Inspect { file } => inspect(file),
    }
}

fn inspect(file: PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(&file)
        .with_context(|| format!("failed to read {}", file.display()))?;
    let run: DescentRun = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse {}", file.display()))?;

    let result = collapse_hypotheses(run.current_scale, &run.hypotheses)?;

    println!("Task: {}", run.task.task_id);
    println!("Goal: {}", run.task.goal);
    println!("Selected hypothesis: {}", result.selected_hypothesis_id);
    println!("Confidence: {:.2}", result.confidence);
    println!("Decision: {:?}", result.decision);
    println!("Reason: {}", result.reason);

    Ok(())
}
