use clap::Parser;
use disk_read_benchmark::benchmarks::benchmark;
use disk_read_benchmark::cli::*;
use disk_read_benchmark::dataset_gathering::*;

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
    }
}
