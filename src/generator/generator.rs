use super::generate_tx::generate_tx;
pub struct Generator {
}
pub struct process_input{
}
impl Generator {
    pub fn generate_from_input(input:i32) -> String{
        if input == 1{
            let (raw_tx, txid) = process_input::generate_tx();
            let final_structured: String = [format!("Raw Transaction 🥩: {}", raw_tx).to_string() , format!("TXID 🪪 : {}", txid).to_string()].join("\n---\n");
            final_structured
        }else if input > 1{
            return process_input::generateblock(input)
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

impl process_input{
    pub fn generate_tx()->(String, String){ //here process the input to choose the type of tx   
        generate_tx::generate_simple_p2wpkh()
    }

    //this will need to call the thrait generate_tx in many ways so maybe needs a file too
    pub fn generateblock(input:i32) -> String {
        format!("Sorry, we cant process more than one transaction in blocks for now so your {} transactions need to wait 😥", input).to_string()
    }
}
