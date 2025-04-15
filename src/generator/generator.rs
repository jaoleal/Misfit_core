
use super::generate_blocks::GenerateBlock;
use super::generate_tx::GenerateTx;
pub struct Generator {}

impl Generator {
    pub fn generate(input: i32) -> String {
        match input {
            1 => {
                let (raw_tx, txid) = GenerateTx::generate_simple_p2wpkh();
                [
                    format!("Raw Transaction 🥩: {}", raw_tx).to_string(),
                    format!("TXID 🪪 : {}", txid).to_string(),
                ]
                .join("\n---\n")
            }

            n if n > 1 => {
                //if the user request more than one transaction we return a block
                let mut raw_tx: Vec<String> = vec![];
                let mut txid: Vec<String> = vec![];

                for _c in 0..input {
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

            _ => "Your input is invalid, try again with a valid number of transactions 😕"
                .to_string(),
        }
    }

    pub fn proces_flags_to_broke(flags: Vec<String>) -> String {
        let mut flags_concateneted = "".to_string();

        for c in flags {
            flags_concateneted += &c;
        }

        format!("When cant process you flags for now {}", flags_concateneted).to_string()
    }

}
