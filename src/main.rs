mod solution;
mod solutions;

use clap::{Parser, Subcommand};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { year, day, part } => {
            solution::run_solutions(year, day, part)
        },
        Commands::List {  } => {
            let solutions = solutions::get_year(solution::CURRENT_YEAR);
            println!("[*] Solutions for {}:", solution::CURRENT_YEAR);

            for (i, e) in solutions.iter().enumerate() {
                println!(
                    " {} Day {}: {}",
                    if i + 1 == solutions.len() {
                        "└"
                    } else {
                        "├"
                    },
                    i + 1,
                    e.name()
                );
            }
        },
    };
}

// https://docs.rs/clap/latest/clap/

/// A cli for AoC
#[derive(Debug, Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code CLI to execute solutions for the challenges", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List the available solutions
    List{},

    /// Run the Advent of Code challenges
    Run{
        // todo add run with execution times

        /// Year of advent of code
        #[arg(value_name = "YEAR", long)]
        year: Option<u32>,

        /// Day of the challenge
        #[arg(value_name = "DAY", long)]
        day: Option<usize>,

        /// Specific part of the challenge
        #[arg(value_name = "PART", long)]
        part: Option<char>,
    },
}
