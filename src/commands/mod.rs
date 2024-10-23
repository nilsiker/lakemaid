use clap::Subcommand;

pub mod class;
pub mod flowchart;

#[derive(Subcommand, Clone)]
pub enum Command {
    Class { file: String },
    Flowchart { file: String },
}
