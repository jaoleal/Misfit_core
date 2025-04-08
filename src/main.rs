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
    /// Specify the number of transactions and optionally break campuses
    Numberoftxs(NumberoftxsArgs),
}

#[derive(Parser)]
struct NumberoftxsArgs {
    /// Number of transactions, defaults to 1
    #[arg(default_value_t = 1)]
    txscount: i32,
    #[command(subcommand)]
    campus_command: Option<CampusSubcommands>,
}

#[derive(Subcommand)]
enum CampusSubcommands {
    /// List campuses to break
    CampusToBreak { listedcampus: Vec<String> },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Numberoftxs(args) => {
            let txscount = args.txscount;
            let input_txs_count = Generator::generate_from_input(txscount);
            
            if let Some(campus_command) = args.campus_command {
                match campus_command {
                    CampusSubcommands::CampusToBreak { listedcampus } => {
                        let input_listedcampus = Generator::proces_flags_to_broke(listedcampus);
                        println!(
                            "Transactions: {}\n---\n Campuses: {}",
                            input_txs_count, input_listedcampus
                        );
                    }
                }
            } else {
                println!("{}", input_txs_count);
            }
        }
    }
}