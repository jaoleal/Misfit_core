use clap::{Parser, Subcommand};
use std::io;
use std::io::Write;
mod generator;
use generator::regtest::RegtestManager;

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
    GetBlockbyHeight {
        height: u64,
    },
    RegtestStart,
    RegtestStop,
    Clear,
    Help,
    Finalize,
}

fn print_help() {
    println!("Available commands:\n");
    println!("new <txscount> [campuses...]  - Generate a transaction, or a block for more than one transaction");
    println!("clear                         - Clear terminal screen");
    println!("get-blockby-height <height>   - Get a block at specific height in the regtest");
    println!("regtest-start                 - Start the regtest node");
    println!("regtest-stop                  - Stop the regtest node(please rember stop before close the program)");
    println!("help                          - Show help message");
    println!("finalize                      - Exit the program\n");
}

fn main() {
    let regtest_manager = RegtestManager::new("bitcoinhos", "-regtest");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        let cli = match Cli::try_parse_from(std::iter::once("").chain(args.iter().copied())) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        match cli.command {
            Commands::New { txscount, campuses } => {
                let transactions = generator::generator::Generator::generate(txscount);
                if !campuses.is_empty() {
                    let processed_campuses = generator::generator::Generator::proces_flags_to_broke(campuses);
                    println!("Transactions: {}\nCampuses: {}", transactions, processed_campuses);
                } else {
                    println!("Transactions: {}", transactions);
                }
            }
            Commands::Clear => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }
            Commands::RegtestStart => handle_result(regtest_manager.start()),
            Commands::RegtestStop => handle_result(regtest_manager.stop()),
            Commands::GetBlockbyHeight { height } => handle_result(regtest_manager.handle_getblockbyheight(height)),
            Commands::Help => print_help(),
            Commands::Finalize => break,
        }
    }
    println!("Program finalized 👋");
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        eprintln!("Error: {} 🚨", e);
    }
}