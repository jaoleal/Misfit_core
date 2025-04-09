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
    New(New),
}

#[derive(Parser)]
struct New {
    /// Number of transactions, defaults to 1
    #[arg(default_value_t = 1)]
    txscount: i32,
    #[command(subcommand)]
    campus_command: Option<CampusSubcommands>,
}

#[derive(Subcommand)]
enum CampusSubcommands {
    /// use with "new "number of txs" update-with-flags"
    UpdateWithFlags { listedcampus: Vec<String> },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New(args) => {
            let txscount = args.txscount;
            let input_txs_count = Generator::generate_from_input(txscount);
            
            if let Some(campus_command) = args.campus_command {
                match campus_command {
                    CampusSubcommands::UpdateWithFlags { listedcampus } => {
                        let input_listedcampus = Generator::proces_flags_to_broke(listedcampus);
                        println!(
                            "New: {}\n---\n Updated with flaws: {}",
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