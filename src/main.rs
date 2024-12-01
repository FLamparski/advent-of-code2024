use clap::{Parser, Subcommand};

mod day1;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {
        /// Path to input data
        input_filename: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day1 { input_filename } => day1::day1(input_filename),
    }
}
