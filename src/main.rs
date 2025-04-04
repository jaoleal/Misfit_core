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
        txscount: i32 
                    // Self anotation, for now the only required input is the number of txs thats define if you take a tx or a block
                    // but in the future the user will need input the campus to break and the kind of transaction... 
    },
    CampusToBreak{
        listedcampus:Vec<String>
    }

}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Numberoftxs { txscount } => {
            let input_txs_count = Generator::generate_from_input(txscount);
            println!("{}", input_txs_count);
        },
        Commands::CampusToBreak { listedcampus } =>{
            let input_listedcampus = Generator::proces_flags_to_broke(listedcampus);
            println!("{}", input_listedcampus)
        }
    }
}