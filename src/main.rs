use clap::{Parser, Subcommand};

mod day1;
mod day2;
mod day3;

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
    Day2 {
        /// Path to input data
        input_filename: String,
    },
    Day3 {
        /// Path to input data
        input_filename: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day1 { input_filename } => day1::day1(input_filename),
        Commands::Day2 { input_filename } => day2::day2(input_filename),
        Commands::Day3 { input_filename } => day3::day3(input_filename),
    }
}
