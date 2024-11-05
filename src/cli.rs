use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Grabs the datasets used for benchmarking
    GrabData,
    ///Runs the benchmark
    Benchmark,
    ///Prepares the directories so other programs can prepare their datasets
    PrepDirs,
    ///Runs it all
    Run,
}
