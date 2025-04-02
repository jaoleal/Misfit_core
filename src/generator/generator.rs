pub struct Generator {
}

impl Generator {
    pub fn generate_from_input(input:i32) -> String{
        if input == 1{
            return process_input::generatetx()
        }else if input > 1{
            return process_input::generateblock(input)
        }else{
            return "Your input is invalid, try again with a valid number of transactions 😕".to_string()
        }
    }
}

pub mod process_input{
    pub fn generatetx() -> String {
        ("Generating a transaction, for now we only generate bitcoin witness p2wpkh transactions").to_string()
        
    }
    pub fn generateblock(input:i32) -> String {
        format!("Sorry, we cant process more than one transaction in Blocks for now so your {} transactions need to wait 😥", input).to_string()
    }
}
