use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Fish, Zsh};
use disk_read_benchmark::benchmarks::benchmark;
use disk_read_benchmark::cli::*;
use disk_read_benchmark::dataset_gathering::*;
use std::io::stdout;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::PrepDirs => {
            // FIXME: okay i'm dumb, this only covers stuff that's not handled by grab_datasets(), and literally nothing creates ext-workdir
            prep_other_dirs();
        }
        Commands::GrabData => {
            grab_datasets().unwrap(); // * should unwrap
        }
        Commands::Benchmark => {
            benchmark();
        }
        Commands::Run => {
            prep_other_dirs();
            grab_datasets().unwrap(); // * should unwrap
            benchmark();
        }
        // I can't be bothered to do this how I *should*, rather than hardcoding it
        Commands::GenerateBashCompletions => {
            generate(
                Bash,
                &mut Cli::command(),
                "disk-read-benchmark",
                &mut stdout(),
            );
        }
        Commands::GenerateZshCompletions => {
            generate(
                Zsh,
                &mut Cli::command(),
                "disk-read-benchmark",
                &mut stdout(),
            );
        }
        Commands::GenerateFishCompletions => {
            generate(
                Fish,
                &mut Cli::command(),
                "disk-read-benchmark",
                &mut stdout(),
            );
        }
    }
}
