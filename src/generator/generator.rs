use bitcoin::consensus::encode;

use super::generate_blocks::GenerateBlock;
use super::generate_tx::{GenerateTx, TxParams};
pub struct Generator {}

impl Generator {
    pub fn block(tx_count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..tx_count {
            let (new_raw_tx, new_txid) = GenerateTx::generate_simple_p2wpkh();
            raw_tx.push(new_raw_tx);
            txid.push(new_txid);
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
            let tx = GenerateTx::random_tx(TxParams {
                version: None,
                lock_time: None,
                input: None,
                output: None,
            });
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

    pub fn _proces_flags_to_broke(flags: Vec<String>) -> String {
        let mut flags_concateneted = "".to_string();

        for c in flags {
            flags_concateneted += &c;
        }

        format!("When cant process you flags for now {}", flags_concateneted).to_string()
    }
}
