use sha2::{Sha256, Digest};
use std::collections::HashSet;
use bitcoin::{Transaction, TxIn, TxOut, Witness, OutPoint, ScriptBuf, Amount};
use bitcoin::blockdata::script::Script;

// Define available invalidation flags
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InvalidationFlag {
    Version,
    InputTxid,
    InputVout,
    InputScriptSig,
    InputSequence,
    OutputAmount,
    OutputScriptPubKey,
    WitnessData,
    Locktime,
    All,
}

impl InvalidationFlag {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "version" => Some(Self::Version),
            "input-txid" | "txid" => Some(Self::InputTxid),
            "input-vout" | "vout" => Some(Self::InputVout),
            "input-script" | "script-sig" => Some(Self::InputScriptSig),
            "input-sequence" | "sequence" => Some(Self::InputSequence),
            "output-amount" | "amount" => Some(Self::OutputAmount),
            "output-script" | "script-pubkey" => Some(Self::OutputScriptPubKey),
            "witness" | "witness-data" => Some(Self::WitnessData),
            "locktime" => Some(Self::Locktime),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

// Transaction processor with Bitcoin Core Transaction struct
pub struct TransactionInvalidator;

impl TransactionInvalidator {
    pub fn invalidate(mut tx: Transaction, flags: &HashSet<InvalidationFlag>) -> Transaction {
        let should_invalidate_all = flags.contains(&InvalidationFlag::All);
        
        // Invalidate transaction structure (affects txid) - do this first
        if should_invalidate_all || flags.contains(&InvalidationFlag::InputTxid) {
            Self::corrupt_txid(&mut tx);
        }
        
        // Invalidate version
        if should_invalidate_all || flags.contains(&InvalidationFlag::Version) {
            tx.version = Self::invalidate_version(tx.version);
        }
        
        // Invalidate locktime
        if should_invalidate_all || flags.contains(&InvalidationFlag::Locktime) {
            tx.lock_time = Self::invalidate_locktime(tx.lock_time);
        }
        
        // Invalidate inputs
        for input in tx.input.iter_mut() {
            Self::invalidate_input_in_place(input, flags, should_invalidate_all);
        }
        
        // Invalidate outputs
        for output in tx.output.iter_mut() {
            Self::invalidate_output_in_place(output, flags, should_invalidate_all);
        }
        
        tx
    }

    fn invalidate_version(v: bitcoin::blockdata::transaction::Version) -> bitcoin::blockdata::transaction::Version {
        bitcoin::blockdata::transaction::Version(v.0 + 1)
    }
    
    fn invalidate_locktime(lt: bitcoin::absolute::LockTime) -> bitcoin::absolute::LockTime {
        match lt {
            bitcoin::absolute::LockTime::Blocks(height) => {
                bitcoin::absolute::LockTime::Blocks(
                    bitcoin::absolute::Height::from_consensus(
                        u32::MAX - height.to_consensus_u32()
                    ).unwrap_or(height)
                )
            },
            bitcoin::absolute::LockTime::Seconds(time) => {
                bitcoin::absolute::LockTime::Seconds(
                    bitcoin::absolute::Time::from_consensus(
                        u32::MAX - time.to_consensus_u32()
                    ).unwrap_or(time)
                )
            },
        }
    }

    fn invalidate_input_in_place(
        input: &mut TxIn, 
        flags: &HashSet<InvalidationFlag>, 
        invalidate_all: bool
    ) {
        // Note: InputTxid invalidation is now handled at transaction level
        
        if invalidate_all || flags.contains(&InvalidationFlag::InputVout) {
            input.previous_output.vout ^= 1; // Flip last bit
        }
        
        // Invalidate script_sig
        if invalidate_all || flags.contains(&InvalidationFlag::InputScriptSig) {
            input.script_sig = Self::corrupt_script(&input.script_sig);
        }
        
        // Invalidate sequence
        if invalidate_all || flags.contains(&InvalidationFlag::InputSequence) {
            input.sequence = bitcoin::Sequence(0xFFFFFFFF ^ input.sequence.0);
        }
        
        // Invalidate witness data
        if invalidate_all || flags.contains(&InvalidationFlag::WitnessData) {
            input.witness = Self::corrupt_witness(&input.witness);
        }
    }

    fn invalidate_output_in_place(
        output: &mut TxOut, 
        flags: &HashSet<InvalidationFlag>, 
        invalidate_all: bool
    ) {
        // Invalidate amount
        if invalidate_all || flags.contains(&InvalidationFlag::OutputAmount) {
            let current_sats = output.value.to_sat();
            output.value = Amount::from_sat(u64::MAX - current_sats);
        }
        
        // Invalidate script_pubkey
        if invalidate_all || flags.contains(&InvalidationFlag::OutputScriptPubKey) {
            output.script_pubkey = Self::corrupt_script(&output.script_pubkey);
        }
    }

    // Helper methods
    fn corrupt_txid(tx: &mut Transaction) -> bitcoin::Txid {
        // Remove the last input if there are multiple inputs
        if tx.input.len() > 1 {
            tx.input.pop();
        } else if !tx.input.is_empty() {
            // If only one input, corrupt its previous output
            tx.input[0].previous_output.vout = tx.input[0].previous_output.vout.wrapping_add(1);
        }
        
        // Compute and return the new transaction ID
        tx.compute_txid()
    }

    fn corrupt_script(script: &ScriptBuf) -> ScriptBuf {
        let mut bytes = script.as_bytes().to_vec();
        if !bytes.is_empty() {
            bytes[0] = bytes[0].wrapping_add(1);
        } else {
            bytes.push(0x51); // Add OP_1 to empty script
        }
        ScriptBuf::from_bytes(bytes)
    }
    
    fn corrupt_witness(witness: &Witness) -> Witness {
        let mut new_witness = witness.clone();
        if let Some(first_item) = new_witness.iter().next() {
            let mut corrupted = first_item.to_vec();
            if !corrupted.is_empty() {
                corrupted[0] = corrupted[0].wrapping_add(1);
            } else {
                corrupted.push(0x01);
            }
            let mut witness_stack = Vec::new();
            witness_stack.push(corrupted);
            // Add remaining items
            for item in witness.iter().skip(1) {
                witness_stack.push(item.to_vec());
            }
            new_witness = Witness::from_slice(&witness_stack);
        } else {
            // Empty witness, add a dummy item
            new_witness = Witness::from_slice(&[vec![0x01]]);
        }
        new_witness
    }
}



fn print_usage() {
    println!("Bitcoin Transaction Invalidator");
    println!("Usage: btc-invalidator [FLAGS]");
    println!("\nAvailable flags:");
    println!("  --all           Invalidate all transaction fields");
    println!("  --version       Invalidate transaction version");
    println!("  --txid          Invalidate input transaction ID");
    println!("  --vout          Invalidate input vout");
    println!("  --script-sig    Invalidate input script signature");
    println!("  --sequence      Invalidate input sequence number");
    println!("  --amount        Invalidate output amount");
    println!("  --script-pubkey Invalidate output script pubkey");
    println!("  --witness       Invalidate witness data");
    println!("  --locktime      Invalidate transaction locktime");
    println!("  --help          Print this help message");
    println!("\nExample: btc-invalidator --txid --amount --locktime");
}

pub fn parse_flags(args: Vec<String>) -> HashSet<InvalidationFlag> {
    let mut flags = HashSet::new();

    // Display help and exit if requested
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        std::process::exit(0);
    }

    // No flags provided - show help
    if args.len() <= 1 {
        print_usage();
        std::process::exit(1);
    }

    // Parse all provided flags
    for arg in args.iter().skip(1) {
        if !arg.starts_with("--") {
            continue;
        }

        let flag_str = &arg[2..]; // Remove "--" prefix
        if let Some(flag) = InvalidationFlag::from_str(flag_str) {
            flags.insert(flag);
        } else {
            println!("Warning: Unknown flag '{}' ignored", arg);
        }
    }

    flags
}