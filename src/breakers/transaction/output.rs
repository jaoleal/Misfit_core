use std::collections::HashSet;
use bitcoin::{TxOut, Amount};
use super::{InvalidationFlag, script::corrupt_script};

pub fn invalidate_output_in_place(
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
        output.script_pubkey = corrupt_script(&output.script_pubkey);
    }
}