pub mod commands;
mod fs;
mod mermaid;
pub mod interop;

use clap::Parser;
use commands::Command;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
/// Lakemaid
///
/// A Mermaid diagram generator.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn run(command: &Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Class { file } => commands::class::exec(file)?,
        Command::Flowchart { file } => commands::flowchart::exec(file)?,
    }

    Ok(())
}
