use super::generate_tx::GenerateTx;
use super::generate_blocks::GenerateBlock;
pub struct Generator {
}
impl Generator {
    pub fn generate_from_input(input:i32) -> String{
        if input == 1{
            let (raw_tx, txid) = GenerateTx::generate_simple_p2wpkh();
            let final_structured: String = [format!("Raw Transaction 🥩: {}", raw_tx).to_string() , format!("TXID 🪪 : {}", txid).to_string()].join("\n---\n");
            final_structured
        }else if input > 1{
            let mut raw_tx:Vec<String> = vec![];
            let mut txid:Vec<String> = vec![];
            for c in 0..input{
                let (new_raw_tx, new_txid) = GenerateTx::generate_simple_p2wpkh();
                raw_tx.push(new_raw_tx);
                txid.push(new_txid);
            }
            let block_header =GenerateBlock::new(raw_tx.clone());
            let final_structured = [format!("Blockheader Info: {:#?} ", block_header),format!("Raw transactions used in it:{:#?}", raw_tx)].join("\n---\n");
            final_structured
            }else{
            return "Your input is invalid, try again with a valid number of transactions 😕".to_string()
        }
    }
    pub fn proces_flags_to_broke(flags:Vec<String>) ->String{
        let mut flags_concateneted = "".to_string();
        for c in flags{
            flags_concateneted += &c;
        }
        return format!("When cant process you flags for now {}",flags_concateneted ).to_string()
    }
}

