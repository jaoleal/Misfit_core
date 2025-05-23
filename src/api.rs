use bitcoin::consensus::encode;
use bitcoin::block::Header;
use bitcoin::Transaction;
use misfit_core::breakers::decoder_tools::{BitcoinTransactionDecoder};
use misfit_core::transaction::random::transaction::TxParams;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::block::generate_blocks::GenerateBlock;
use misfit_core::breakers::{decoder_tools,block_breaker, transaction_breaker};

pub struct Generator {}

impl Generator {
    pub fn block(tx_count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..tx_count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        let block_header = GenerateBlock::new(txid.clone());

        [
            format!("Blockheader Info 🧊: {:#?} ", block_header),
            format!("Raw transactions used in it:{:#?}", raw_tx),
            format!("Used Txids: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    // TODO: Implement params into transaction generator
    pub fn transaction(count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        [
            format!("Raw Transactions: {:#?}", raw_tx),
            format!("TXIDs: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    pub fn decode_raw_transaction(raw_tx:String) -> Result<Transaction,Box<dyn std::error::Error>> {
        let decoder = decoder_tools::BitcoinTransactionDecoder::new();
        let decoded = decoder.decode_hex(&raw_tx);
        decoded    
    }

    pub fn decoder_block_header(block_header:String) -> Result<Header, Box<dyn std::error::Error>> {
        decoder_tools::BlockUtils::decode_header_from_hex(&block_header)
    }

    pub fn break_transaction(transaction:String, flags:Vec<String>)-> String{
            // Parse command line flags
    let flags = transaction_breaker::parse_flags(flags);
    
    if flags.is_empty() {
        println!("No invalidation flags specified. Use --help for usage information.");
    }
    let decoded_tx = Generator::decode_raw_transaction(transaction.clone());
    // Create invalid version based on specified flags
    let invalid_tx = transaction_breaker::TransactionInvalidator::invalidate(decoded_tx.unwrap(), &flags);

    // List which fields are being invalidated
    println!("Invalidating the following fields:");
    for flag in &flags {
        if *flag == transaction_breaker::InvalidationFlag::All {
            println!("  - ALL FIELDS");
            break;
        }
        match flag {
            transaction_breaker::InvalidationFlag::Version => println!("  - Transaction Version"),
            transaction_breaker::InvalidationFlag::InputTxid => println!("  - Input TXIDs"),
            transaction_breaker::InvalidationFlag::InputVout => println!("  - Input Vouts"),
            transaction_breaker::InvalidationFlag::InputScriptSig => println!("  - Input Script Signatures"),
            transaction_breaker::InvalidationFlag::InputSequence => println!("  - Input Sequences"),
            transaction_breaker::InvalidationFlag::OutputAmount => println!("  - Output Amounts"),
            transaction_breaker::InvalidationFlag::OutputScriptPubKey => println!("  - Output Script PubKeys"),
            transaction_breaker::InvalidationFlag::WitnessData => println!("  - Witness Data"),
            transaction_breaker::InvalidationFlag::Locktime => println!("  - Locktime"),
            _ => {}
        }
    }

    // Display results
    println!("\n Inputed Transaction:\n{}\n", transaction);
    format!("{:#?}",invalid_tx)
    }
/* 
    pub fn break_block(flags:Vec<String>)-> String{

    }
*/
}
