use bitcoin::Block;
use super::random::block::{BlockParams, RandomBlock};
use bitcoin::{absolute::LockTime, transaction::Version, TxOut, ScriptBuf, Sequence, Witness, };

use crate::transaction::{
        generator::GenerateTx,
        random::{output::OutputParams, input::InputParams, transaction::TxParams}, };

pub struct GenerateBlock {}

impl GenerateBlock {

    pub fn valid_random(mut params: BlockParams) -> Block {
        let null_value  = TxOut::NULL;
        let coinbase_input = InputParams {
            outpoint: None, // Coinbase transactions don't spend existing outputs
            script: Some(ScriptBuf::new()), // Empty script or coinbase data
            sequence: Some(Sequence::MAX), // Typically 0xFFFFFFFF for coinbase
            witness: Some(Witness::new()), // Empty witness for coinbase
        };       
        let coinbase_output = OutputParams  {
            value: Some(null_value.value),
            script_pubkey: Some(null_value.script_pubkey),
            script_params: None,
        };
        //following a gambiarra to made the coinbase
        let coinbase_params = TxParams {
            version: Some(Version::ONE),
            lock_time: Some(LockTime::ZERO),
            input: Some(coinbase_input), 
            output:Some(coinbase_output),
        };        

        let coinbase = GenerateTx::valid_random(coinbase_params);

        match params.txs {
            Some(ref mut txs) => {
                txs.insert(0, coinbase);
            }
            _none => {
                params.txs = Some(vec![coinbase]);
            }
        }
        Block::random(params)
    }
}