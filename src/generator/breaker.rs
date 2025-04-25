
pub struct breaker_by_flags{

}
pub struct inputs {
    _txid: String,
    _vout: u32,
    _scriptsig_size: u32,
    _scriptsig: String,
    _sequence: u32,
}

pub struct outputs {
    _amount: u32,
    _scriptpubkeysize:u32,
    _scriptpubkey:String,
}

pub struct witness {
    stack_items: u32,//the count of items
    size: u32,//the size of the next item
    item: String,
}

pub struct is_a_transaction{
    pub _version: u32,
    pub _marker: u8, // Used to indicate a segwit transaction. Must be 00.
    pub _flag: u8, // Used to indicate a segwit transaction. Must be 01 or greater.
    pub _input_count: u32,
    pub _inputs: inputs,
    pub _output_count: u32,
    pub _outputs: outputs,
    pub witness: witness,
    pub _locktime: u32,

}

pub struct is_a_block{
    pub _version: u32,
    pub _prev_block_hash: String,
    pub _merkle_root: String,
    pub _timestamp: u32,
    pub _bits: u32,
    pub _nonce: u32,
    pub _transaction_count: u32,
    pub _block_header: String,
}
impl breaker_by_flags {
     pub fn is_a_transaction(campus:is_a_transaction){
        //here the ifs for choice the func in transaction to call
        transaction_process(campus)
     }
     pub fn is_a_block(campus:is_a_block){
        // same thing but for the block process call
            block_process(campus)
     }
}


impl transaction_process{
    //here we process the flags referenciating in the struct
    fn version(campus: _version) -> u32{
         //
    }
}

impl block_process{
    //same here but calling transaction process for every transaction in the block if nescessary
}