use bitcoin::{
    consensus::{deserialize},
    Transaction, OutPoint, 
    Txid, ScriptBuf,
};
use bitcoin::{
    blockdata::{
        block::{Block, Header},
    },
};
use bitcoin::consensus::Decodable;
#[derive(Debug)]
pub struct DecodedTransaction {
    pub txid: Txid,
    pub version: i32,
    pub lock_time: u32,
    pub inputs: Vec<DecodedInput>,
    pub outputs: Vec<DecodedOutput>,
    pub weight: u64,
    pub vsize: u64,
    pub size: usize,
}

#[derive(Debug)]
pub struct DecodedInput {
    pub previous_output: OutPoint,
    pub script_sig: ScriptBuf,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct DecodedOutput {
    pub value: u64,
    pub script_pubkey: ScriptBuf,
}

pub struct BitcoinTransactionDecoder;

impl BitcoinTransactionDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_hex(&self, hex_string: &str) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let clean_hex = hex_string.trim().replace(" ", "").to_lowercase();
        
        let bytes = hex::decode(&clean_hex)?;
        
        self.decode_bytes(&bytes)
    }

    pub fn decode_bytes(&self, bytes: &[u8]) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let tx: Transaction = deserialize(bytes)?;
        
        self.decode_transaction(tx, bytes.len())
    }

    /// Convert a Transaction struct to our decoded format
    fn decode_transaction(&self, tx: Transaction, size: usize) -> Result<DecodedTransaction, Box<dyn std::error::Error>> {
        let txid = tx.compute_txid();
        let weight = tx.weight().to_wu();
        let vsize = tx.vsize() as u64;

        let inputs: Vec<DecodedInput> = tx.input.into_iter().map(|input| {
            DecodedInput {
                previous_output: input.previous_output,
                script_sig: input.script_sig,
                sequence: input.sequence.0,
                witness: input.witness.to_vec(),
            }
        }).collect();

        let outputs: Vec<DecodedOutput> = tx.output.into_iter().map(|output| {
            DecodedOutput {
                value: output.value.to_sat(),
                script_pubkey: output.script_pubkey,
            }
        }).collect();

        Ok(DecodedTransaction {
            txid,
            version: tx.version.0,
            lock_time: tx.lock_time.to_consensus_u32(),
            inputs,
            outputs,
            weight,
            vsize,
            size,
        })
    }
/* 
    /// Pretty print a decoded transaction
    pub fn print_transaction(&self, decoded: &DecodedTransaction) {
        println!("=== Bitcoin Transaction Details ===");
        println!("TXID: {}", decoded.txid);
        println!("Version: {}", decoded.version);
        println!("Lock Time: {}", decoded.lock_time);
        println!("Size: {} bytes", decoded.size);
        println!("Virtual Size: {} vbytes", decoded.vsize);
        println!("Weight: {} WU", decoded.weight);
        println!();

        println!("INPUTS ({}):", decoded.inputs.len());
        for (i, input) in decoded.inputs.iter().enumerate() {
            println!("  Input {}:", i);
            println!("    Previous Output: {}:{}", input.previous_output.txid, input.previous_output.vout);
            println!("    Script Sig: {}", input.script_sig);
            println!("    Sequence: 0x{:08x}", input.sequence);
            
            if !input.witness.is_empty() {
                println!("    Witness ({} items):", input.witness.len());
                for (j, witness_item) in input.witness.iter().enumerate() {
                    println!("      {}: {}", j, hex::encode(witness_item));
                }
            }
            println!();
        }

        println!("OUTPUTS ({}):", decoded.outputs.len());
        for (i, output) in decoded.outputs.iter().enumerate() {
            println!("  Output {}:", i);
            println!("    Value: {} satoshis ({} BTC)", output.value, output.value as f64 / 100_000_000.0);
            println!("    Script PubKey: {}", output.script_pubkey);
            println!();
        }
    }
*/

}




// Decoding, utilities, and helper functions implementation
pub struct BlockUtils;
impl BlockUtils {
    // Utility method to decode block header from hex string
    pub fn decode_header_from_hex(hex_string: &str) -> Result<Header, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        if bytes.len() != 80 {
            return Err(format!("Invalid header length: expected 80 bytes, got {}", bytes.len()).into());
        }
        let header = Header::consensus_decode(&mut &bytes[..])?;
        Ok(header)
    }

    // Utility method to decode block from hex string
    pub fn decode_block_from_hex(hex_string: &str) -> Result<Block, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_string)?;
        let block = Block::consensus_decode(&mut &bytes[..])?;
        Ok(block)
    }

    // Create a minimal block from a header (for testing purposes)
    pub fn create_minimal_block_from_header(header: Header) -> Block {
        Block {
            header,
            txdata: vec![], // Empty transaction list
        }
    }

    // Print block header information
    pub fn print_header_info(header: &Header, label: &str) {
        println!("\n=== {} ===", label);
        println!("Version: {}", header.version.to_consensus());
        println!("Previous Block: {}", header.prev_blockhash);
        println!("Merkle Root: {}", header.merkle_root);
        println!("Timestamp: {}", header.time);
        println!("Bits: 0x{:08x}", header.bits.to_consensus());
        println!("Nonce: {}", header.nonce);
        println!("Block Hash: {}", header.block_hash());
    }

    // Encode header to hex string
    pub fn encode_header_to_hex(header: &Header) -> String {
        use bitcoin::consensus::Encodable;
        let mut bytes = Vec::new();
        header.consensus_encode(&mut bytes).expect("Failed to encode header");
        hex::encode(bytes)
    }

    // Compare two headers and show differences
    pub fn compare_headers(original: &Header, modified: &Header) {
        println!("\n=== HEADER COMPARISON ===");
        
        if original.version != modified.version {
            println!("Version: {} → {}", 
                original.version.to_consensus(), 
                modified.version.to_consensus());
        }
        
        if original.prev_blockhash != modified.prev_blockhash {
            println!("Prev Block Hash: {} → {}", 
                original.prev_blockhash, 
                modified.prev_blockhash);
        }
        
        if original.merkle_root != modified.merkle_root {
            println!("Merkle Root: {} → {}", 
                original.merkle_root, 
                modified.merkle_root);
        }
        
        if original.time != modified.time {
            println!("Timestamp: {} → {}", 
                original.time, 
                modified.time);
        }
        
        if original.bits != modified.bits {
            println!("Bits: 0x{:08x} → 0x{:08x}", 
                original.bits.to_consensus(),
                modified.bits.to_consensus());
        }
        
        if original.nonce != modified.nonce {
            println!("Nonce: {} → {}", 
                original.nonce, 
                modified.nonce);
        }
        
        println!("Block Hash: {} → {}", 
            original.block_hash(), 
            modified.block_hash());
    }
}

// Simplified interface for common use cases


//Todo implement the following for the cli 
/* 
// Example usage and tests
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let decoder = BitcoinTransactionDecoder::new();

    // Example raw transaction (this is a mainnet transaction)
    let raw_tx_hex = "01000000000101d7fc103aeb1e32e125959328597717f83c6de279da205de2cd52472f726171040100000000ffffffff02180114000000000017a914aeb0efc1da63629651dc3322c092c6607937c87c87e8af4d7a000000001600141ce75726e812b2fcaf36d6a178ccbfd58a5efcd602483045022100d91d64b5b0326b83d1cfca891a6df291ba975c43c51abfa0f021d9733fe69d6a02206061089696fb44643c4e6e4311304d6d4c41309c10eba835c2835ced06537e960121021b7f2cb05643404c57d0587b48c8d882a454f1040c47cbd31c73d29b599d040100000000";

    println!("Decoding raw transaction...\n");
    
    match decoder.decode_hex(raw_tx_hex) {
        Ok(decoded) => {
            decoder.print_transaction(&decoded);
        }
        Err(e) => {
            eprintln!("Error decoding transaction: {}", e);
        }
    }



    Ok(())
}

*/