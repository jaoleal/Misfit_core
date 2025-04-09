use clap::{Parser, Subcommand};
use std::io;
use std::io::Write;
mod generator;
use generator::generator::Generator;

#[derive(Parser)]
#[command(version, about, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(default_value_t = 1)]
        txscount: i32,
        campuses: Vec<String>,
    },
    Clear,
    Help,
    Finalize,
}

fn print_help() {
    println!("Available commands:\n");
    println!("new <txscount> [campuses...]  - generate a transaction, or a block for more than one transaction");
    println!("clear                         - Clear terminal screen");
    println!("help                          - give you some help");
    println!("finalize                      - Exit the program\n");
}

fn main() {
    loop {
        println!("Enter command ('help' for options, 'finalize' to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        let cli = match Cli::try_parse_from(std::iter::once("").chain(args.iter().copied())) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        match cli.command {
            Commands::New { txscount, campuses } => {
                let transactions = Generator::generate_from_input(txscount);
                
                if !campuses.is_empty() {
                    let processed_campuses = Generator::proces_flags_to_broke(campuses);
                    println!("Transactions: {}\nCampuses: {}", transactions, processed_campuses);
                } else {
                    println!("Transactions: {}", transactions);
                }
            }
            Commands::Clear => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }
            Commands::Help => {
                print_help();
            }
            Commands::Finalize => break,
        }
    }
    println!("Program finalized");
}