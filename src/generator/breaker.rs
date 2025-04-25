pub struct breaker_by_flags;

pub struct inputs {
    _txid: String,
    _vout: u32,
    _scriptsig_size: u32,
    _scriptsig: String,
    _sequence: u32,
}

pub struct outputs {
    _amount: u32,
    _scriptpubkeysize: u32,
    _scriptpubkey: String,
}

pub struct witness {
    stack_items: u32,
    size: u32,
    item: String,
}

pub struct is_a_transaction {
    pub _version: u32,
    pub _marker: u8,
    pub _flag: u8,
    pub _input_count: u32,
    pub _inputs: inputs,
    pub _output_count: u32,
    pub _outputs: outputs,
    pub witness: witness,
    pub _locktime: u32,
}

pub struct is_a_block {
    pub _version: u32,
    pub _prev_block_hash: String,
    pub _merkle_root: String,
    pub _timestamp: u32,
    pub _bits: u32,
    pub _nonce: u32,
    pub _transaction_count: u32,
    pub _block_header: String,
}

// Transaction processing implementation
pub struct TransactionProcessor;

impl TransactionProcessor {
    pub fn process_version(_version: u32) {
        //anything diferent of the version of that current type of tx, sooo maybe we can just add o remove one from this 
    }
    pub fn process_marker(_marker: u8) {
        //Used to indicate a segwit transaction. Must be 00. sooo just diferent of 00? 
    }
    pub fn process_flag(_flag: u8) {
        //Used to indicate a segwit transaction. Must be 01 or greater. 00?
    }
    pub fn process_input_count(_input_count: u32) {
        // i think here we can add or delete a unity here too.
    }
    
    pub fn process_inputs(inputs: inputs) {
        Self::process_txid(inputs._txid);
        Self::process_vout(inputs._vout);
        Self::process_scriptsig_size(inputs._scriptsig_size);
        Self::process_scriptsig(inputs._scriptsig);
        Self::process_sequence(inputs._sequence);
    }
    
    pub fn process_txid(_txid: String) {
        // a dont exist or malformed txid?
    }
    pub fn process_vout(_vout: u32) {
        //The index number of the output you want to spend, just change it too
    }
    pub fn process_scriptsig_size(_size: u32) {
        //incompatible for the next item too
    }
    pub fn process_scriptsig(_scriptsig: String) {
        // maybe the same of scriptpubkey
    }
    pub fn process_sequence(_sequence: u32) {
        //here i will probably need to use invalid in a syntax way...
    }
    
    pub fn process_output_count(_output_count: u32) {
        //same logic of process_input_count. 
    }
    
    pub fn process_outputs(outputs: outputs) {
        Self::process_amount(outputs._amount);
        Self::process_scriptpubkeysize(outputs._scriptpubkeysize);
        Self::process_scriptpubkey(outputs._scriptpubkey);
    }
    
    pub fn process_amount(_amount: u32) {
        //this will REALY BE hard but probably this and txid will need a real blockchain....
    }
    pub fn process_scriptpubkeysize(_size: u32) {
        // here i think i can just put a incompatible with the next item...
    }
    pub fn process_scriptpubkey(_scriptpubkey: String) {
       //maybe a malformed script or dont existed
    }
    
    pub fn process_witness(witness: witness) {
        Self::process_stack_items(witness.stack_items);
        Self::process_witness_size(witness.size);
        Self::process_witness_item(witness.item);
    }
    
    pub fn process_stack_items(_items: u32) {
        // The number of items to be pushed on to the stack as part of the unlocking code. maybe same of size...
    }
    pub fn process_witness_size(_size: u32) {
        //same logic of all the size campus in general
    }
    pub fn process_witness_item(_item: String) {
     //maybe just dont exist or malformed witness item
    } 
    pub fn process_locktime(_locktime: u32) {
    // dont know how to do witout syntax error
    }
}

// Block processing implementation
pub struct BlockProcessor;

impl BlockProcessor {
    pub fn process_version(_version: u32) {
        //same logic of the transaction version
    }
    pub fn process_prev_block_hash(_hash: String) {
        //herer i can create just a random block hash?
    }
    pub fn process_merkle_root(_root: String) {
        //here is easy too just add transactions in the root that are not in the block or remove some of tham
    }
    pub fn process_timestamp(_timestamp: u32) {
        //The current time as a Unix timestamp. realy dont know how the people will verify this....
    }
    pub fn process_bits(_bits: u32) {
        //A compact representation of the current target i think i can just change this value and it will be invalid
    }
    pub fn process_nonce(_nonce: u32) {
        //REAL dont know....
    }
    pub fn process_transaction_count(_count: u32) {
        // same logic of the sizes campus
    }
    pub fn process_block_header(_header: String) {
        //here a not literal in the block but just create someting incompatible with the real block
    }
}

impl breaker_by_flags {
    pub fn is_a_transaction(campus: is_a_transaction) {
        TransactionProcessor::process_version(campus._version);
        TransactionProcessor::process_marker(campus._marker);
        TransactionProcessor::process_flag(campus._flag);
        TransactionProcessor::process_input_count(campus._input_count);
        TransactionProcessor::process_inputs(campus._inputs);
        TransactionProcessor::process_output_count(campus._output_count);
        TransactionProcessor::process_outputs(campus._outputs);
        TransactionProcessor::process_witness(campus.witness);
        TransactionProcessor::process_locktime(campus._locktime);
    }

    pub fn is_a_block(campus: is_a_block) {
        BlockProcessor::process_version(campus._version);
        BlockProcessor::process_prev_block_hash(campus._prev_block_hash);
        BlockProcessor::process_merkle_root(campus._merkle_root);
        BlockProcessor::process_timestamp(campus._timestamp);
        BlockProcessor::process_bits(campus._bits);
        BlockProcessor::process_nonce(campus._nonce);
        BlockProcessor::process_transaction_count(campus._transaction_count);
        BlockProcessor::process_block_header(campus._block_header);
    }
}