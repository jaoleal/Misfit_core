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
    Tx {
        #[arg(short, long)]
        input: String
    },
    Block {
        #[arg(short, long)]
        data: String
    },
}

fn main() {
    let cli = Cli::parse();
    let generator = Generator::new();

    match cli.command {
        Commands::Tx { input } => {
            let result = generator.generatetx(input);
            println!("{}", result);
        }
        Commands::Block { data } => {
            let result = generator.generateblock(data);
            println!("{}", result);
        }
    }
}