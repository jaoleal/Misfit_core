use bitcoin::consensus::encode;
use bitcoin::block::Header;
use bitcoin::Transaction;
use std::collections::HashSet;
use misfit_core::transaction::random::transaction::TxParams;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::block::generate_blocks::GenerateBlock;
use misfit_core::breakers::{decoder_tools, transaction_breaker};

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

    pub fn decode_raw_transaction(raw_tx: String) -> Result<Transaction, Box<dyn std::error::Error>> {
        let decoder = decoder_tools::BitcoinTransactionDecoder::new();
        let decoded = decoder.decode_hex(&raw_tx);
        decoded    
    }

    pub fn decoder_block_header(block_header: String) -> Result<Header, Box<dyn std::error::Error>> {
        decoder_tools::BlockUtils::decode_header_from_hex(&block_header)
    }

    pub fn break_transaction(transaction: String, cli_flags: Vec<String>) -> String {
        // Convert CLI flags to InvalidationFlag HashSet
        let invalidation_flags = Self::parse_cli_flags_to_invalidation_flags(cli_flags);
        
        if invalidation_flags.is_empty() {
            return "No invalidation flags specified. Use 'help' for usage information.".to_string();
        }

        // Decode the transaction
        let decoded_tx = match Self::decode_raw_transaction(transaction.clone()) {
            Ok(tx) => tx,
            Err(e) => return format!("Error decoding transaction: {}", e),
        };

        // Create invalid version based on specified flags
        let invalid_tx = transaction_breaker::TransactionInvalidator::invalidate(decoded_tx, &invalidation_flags);

        // Build the result string
        let mut result = String::new();
        
        // List which fields are being invalidated
        result.push_str("Invalidating the following fields:\n");
        
        if invalidation_flags.contains(&transaction_breaker::InvalidationFlag::All) {
            result.push_str("  - ALL FIELDS\n");
        } else {
            for flag in &invalidation_flags {
                match flag {
                    transaction_breaker::InvalidationFlag::Version => result.push_str("  - Transaction Version\n"),
                    transaction_breaker::InvalidationFlag::InputTxid => result.push_str("  - Input TXIDs\n"),
                    transaction_breaker::InvalidationFlag::InputVout => result.push_str("  - Input Vouts\n"),
                    transaction_breaker::InvalidationFlag::InputScriptSig => result.push_str("  - Input Script Signatures\n"),
                    transaction_breaker::InvalidationFlag::InputSequence => result.push_str("  - Input Sequences\n"),
                    transaction_breaker::InvalidationFlag::OutputAmount => result.push_str("  - Output Amounts\n"),
                    transaction_breaker::InvalidationFlag::OutputScriptPubKey => result.push_str("  - Output Script PubKeys\n"),
                    transaction_breaker::InvalidationFlag::WitnessData => result.push_str("  - Witness Data\n"),
                    transaction_breaker::InvalidationFlag::Locktime => result.push_str("  - Locktime\n"),
                    _ => {}
                }
            }
        }

        // Display results
        result.push_str(&format!("\nInputed Transaction:\n{}\n\n", transaction));
        result.push_str(&format!("Invalidated Transaction:\n{:#?}", invalid_tx));
        
        result
    }

    /// Convert CLI flags (from clap) to InvalidationFlag HashSet
    fn parse_cli_flags_to_invalidation_flags(cli_flags: Vec<String>) -> HashSet<transaction_breaker::InvalidationFlag> {
        let mut flags = HashSet::new();

        for flag in cli_flags {
            let invalidation_flag = match flag.as_str() {
                "--version" => Some(transaction_breaker::InvalidationFlag::Version),
                "--txid" => Some(transaction_breaker::InvalidationFlag::InputTxid),
                "--vout" => Some(transaction_breaker::InvalidationFlag::InputVout),
                "--script-sig" => Some(transaction_breaker::InvalidationFlag::InputScriptSig),
                "--sequence" => Some(transaction_breaker::InvalidationFlag::InputSequence),
                "--amount" => Some(transaction_breaker::InvalidationFlag::OutputAmount),
                "--script-pubkey" => Some(transaction_breaker::InvalidationFlag::OutputScriptPubKey),
                "--witness" => Some(transaction_breaker::InvalidationFlag::WitnessData),
                "--locktime" => Some(transaction_breaker::InvalidationFlag::Locktime),
                "--all" => Some(transaction_breaker::InvalidationFlag::All),
                _ => {
                    println!("Warning: Unknown flag '{}' ignored", flag);
                    None
                }
            };

            if let Some(flag) = invalidation_flag {
                flags.insert(flag);
            }
        }

        flags
    }

/* 
    pub fn break_block(flags: Vec<String>) -> String {
        // TODO: Implement block breaking functionality
        "Block breaking not yet implemented".to_string()
    }
*/
}