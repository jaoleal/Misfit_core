use super::generate_tx::GenerateTx;
use super::generate_blocks::GenerateBlock;
pub struct Generator {
}
pub struct ProcessInput{
}
impl Generator {
    pub fn generate_from_input(input:i32) -> String{
        if input == 1{
            let (raw_tx, txid) = ProcessInput::generate_tx();
            let final_structured: String = [format!("Raw Transaction 🥩: {}", raw_tx).to_string() , format!("TXID 🪪 : {}", txid).to_string()].join("\n---\n");
            final_structured
        }else if input > 1{
            return "not making blocks yet".to_string()
            }else{
            return "Your input is invalid, try again with a valid number of transactions 😕".to_string()
        }
    }
    pub fn proces_flags_to_broke(flags:Vec<String>) ->String{
        let mut flags_concateneted = "".to_string();
        for c in flags{
            flags_concateneted += &c;
        }
        return format!("Wen cant process you flags for now {}",flags_concateneted ).to_string()
    }
}

impl ProcessInput{
    pub fn generate_tx()->(String, String){ //here process the input to choose the type of tx   
        GenerateTx::generate_simple_p2wpkh()
    }

    //this will need to call the thrait generate_tx in many ways so maybe needs a file too
    pub fn generateblock(txids:Vec<String>) -> String {
        let block_header = GenerateBlock::new(txids);
        format!("{:#?}",block_header)
    }
}
