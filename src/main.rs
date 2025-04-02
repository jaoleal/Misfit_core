use clap::{Parser, Subcommand};
mod generator;
use generator::generator::Generator;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Numberoftxs {
        input1: i32 // Positional argument (no flags)
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Numberoftxs { input1 } => {
            let result = Generator::generate_from_input(input1);
            println!("{}", result);
        }
    }
}