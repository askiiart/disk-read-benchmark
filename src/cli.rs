use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Generate bash completions
    GenerateBashCompletions,
    ///Generate zsh completions
    GenerateZshCompletions,
    ///Generate fish completions
    GenerateFishCompletions,
    ///Grabs the datasets used for benchmarking
    GrabData,
    ///Runs the benchmark
    Benchmark,
    ///Prepares the directories so other programs can prepare their datasets
    PrepDirs,
    ///Runs it all
    Run,
}
