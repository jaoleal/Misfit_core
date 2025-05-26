use std::io;
use std::io::Write;

use clap::{Parser, Subcommand};

use misfit_core::regtest_pack::regtest::RegtestManager;
use crate::api::Generator;

#[derive(Parser)]
#[command(version, about, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Help,
    Clear,
    Exit,
    #[command(name = "decode-transaction")]
    DecodeTransaction {
        raw_transaction: String
    },
    #[command(name = "decode-block")]
    DecodeBlock {
        block_header: String
    },
    #[command(name = "break-transaction")]
    BreakTransaction {
        raw_transaction: String,
        #[arg(long, help = "Invalidate transaction version")]
        version: bool,
        #[arg(long, help = "Invalidate input transaction ID")]
        txid: bool,
        #[arg(long, help = "Invalidate input vout")]
        vout: bool,
        #[arg(long = "script-sig", help = "Invalidate input script signature")]
        script_sig: bool,
        #[arg(long, help = "Invalidate input sequence number")]
        sequence: bool,
        #[arg(long, help = "Invalidate output amount")]
        amount: bool,
        #[arg(long = "script-pubkey", help = "Invalidate output script pubkey")]
        script_pubkey: bool,
        #[arg(long, help = "Invalidate witness data")]
        witness: bool,
        #[arg(long, help = "Invalidate transaction locktime")]
        locktime: bool,
        #[arg(long, help = "Invalidate all transaction fields")]
        all: bool,
    },
    Tx {
        #[arg(default_value_t = 1)]
        txscount: u32,
        campuses: Vec<String>,
    },
    Block {
        #[arg(default_value_t = 1)]
        txscount: u32,
    },
    #[command(name = "regtest-start")]
    RegtestStart,
    #[command(name = "regtest-stop")]
    RegtestStop,
    #[command(name = "get-blockby-height")]
    GetBlockbyHeight {
        height: u64,
    },
}

pub fn handle() {
    let regtest_manager = RegtestManager::new("bitcoinhos", "-regtest");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
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
            Commands::Help => help(),
            Commands::DecodeTransaction { raw_transaction } => transaction_splitter(raw_transaction),
            Commands::DecodeBlock { block_header } => block_splitter(block_header),
            Commands::BreakTransaction { 
                raw_transaction, 
                version, 
                txid, 
                vout, 
                script_sig, 
                sequence, 
                amount, 
                script_pubkey, 
                witness, 
                locktime, 
                all 
            } => {
                let flags = build_flags_vector(
                    version, txid, vout, script_sig, sequence, 
                    amount, script_pubkey, witness, locktime, all
                );
                break_transaction(raw_transaction, flags);
            },
            Commands::Tx { txscount, .. } => transaction(txscount), // TODO: Implement params into transaction generator
            Commands::Block { txscount } => block(txscount),
            Commands::Clear => clear(),
            Commands::RegtestStart => handle_result(regtest_manager.start()),
            Commands::RegtestStop => handle_result(regtest_manager.stop()),
            Commands::GetBlockbyHeight { height } => {
                handle_result(regtest_manager.handle_getblockbyheight(height))
            }
            Commands::Exit => break
        }
    }
    println!("Program finalized 👋");
}

fn build_flags_vector(
    version: bool, 
    txid: bool, 
    vout: bool, 
    script_sig: bool, 
    sequence: bool, 
    amount: bool, 
    script_pubkey: bool, 
    witness: bool, 
    locktime: bool, 
    all: bool
) -> Vec<String> {
    let mut flags = Vec::new();
    
    if all {
        flags.push("--all".to_string());
        return flags;
    }
    
    if version { flags.push("--version".to_string()); }
    if txid { flags.push("--txid".to_string()); }
    if vout { flags.push("--vout".to_string()); }
    if script_sig { flags.push("--script-sig".to_string()); }
    if sequence { flags.push("--sequence".to_string()); }
    if amount { flags.push("--amount".to_string()); }
    if script_pubkey { flags.push("--script-pubkey".to_string()); }
    if witness { flags.push("--witness".to_string()); }
    if locktime { flags.push("--locktime".to_string()); }
    
    flags
}

fn help() {
    println!("Available commands:\n");
    println!("[Utils]");
    println!("help                                  - Show help message");
    println!("clear                                 - Clear terminal screen");
    println!("exit");
    println!("");
    println!("[Decode]");
    println!("decode-transaction <raw_tx>           - Decode a raw transaction");
    println!("decode-block <block_header>           - Decode a block header");
    println!("");
    println!("[Break/Invalidate]");
    println!("break-transaction <raw_tx> [FLAGS]    - Break/invalidate specific fields of a transaction");
    println!("  Available flags:");
    println!("    --version         - Invalidate transaction version");
    println!("    --txid            - Invalidate input transaction ID");
    println!("    --vout            - Invalidate input vout");
    println!("    --script-sig      - Invalidate input script signature");
    println!("    --sequence        - Invalidate input sequence number");
    println!("    --amount          - Invalidate output amount");
    println!("    --script-pubkey   - Invalidate output script pubkey");
    println!("    --witness         - Invalidate witness data");
    println!("    --locktime        - Invalidate transaction locktime");
    println!("    --all             - Invalidate all transaction fields");
    println!("");
    println!("[Generate]");
    println!("tx <txscount> [params...]             - Generate one or more transactions");
    println!(
        "block <txscount>                      - Generate new block with one or more transactions"
    );
    println!("");
    println!("[Regtest]");
    println!(
        "get-blockby-height <height>           - Get a block at specific height in the regtest"
    );
    println!("regtest-start                         - Start the regtest node");
    println!("regtest-stop                          - Stop the regtest node (please remember to stop before closing the program)");
}

fn transaction_splitter(raw_transaction: String) {
    match Generator::decode_raw_transaction(raw_transaction) {
        Ok(decoded) => {
            println!("Version: {}", decoded.version);
            println!("Locktime: {}", decoded.lock_time);
            println!("Input count: {:#?}", decoded.input);
            println!("Output count: {:#?}", decoded.output);
        },
        Err(e) => {
            eprintln!("Error decoding transaction: {}", e);
        }
    }
}

fn block_splitter(block_header: String) {
    match Generator::decoder_block_header(block_header) {
        Ok(header) => {
            println!("Version: {}", header.version.to_consensus());
            println!("Previous Block: {}", header.prev_blockhash);
            println!("Merkle Root: {}", header.merkle_root);
            println!("Timestamp: {}", header.time);
            println!("Bits: 0x{:08x}", header.bits.to_consensus());
            println!("Nonce: {}", header.nonce);
            println!("Block Hash: {}", header.block_hash());
        },
        Err(e) => {
            eprintln!("Error decoding block header: {}", e);
        }
    }
}

fn break_transaction(raw_transaction: String, flags: Vec<String>) {
    if flags.is_empty() {
        println!("No invalidation flags specified. Use 'help' for usage information.");
        return;
    }
    
    let result = Generator::break_transaction(raw_transaction, flags);
    println!("🔨 Transaction Breaking Result:");
    println!("{}", result);
}

fn transaction(txscount: u32) {
    let transactions = Generator::transaction(txscount);
    println!("Transactions: {}", transactions);
}

fn block(txscount: u32) {
    let block = Generator::block(txscount);
    println!("Block: {}", block);
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        eprintln!("Error: {} 🚨", e);
    }
}