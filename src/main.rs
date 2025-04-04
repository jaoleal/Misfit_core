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
        input1: i32 // Self anotation, for now the only required input is the number of txs thats define if you take a tx or a block
                    // but in the future the user will need input the campus to break and the kind of transaction... 
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