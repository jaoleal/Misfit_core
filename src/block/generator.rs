use bitcoin::Block;
use super::random::block::{BlockParams, RandomBlock};
use crate::{ 
    transaction::{
        generator::GenerateTx,
        random::{transaction::TxParams}
    }
};

pub struct GenerateBlock {}

impl GenerateBlock {
    pub fn valid_random(mut params: BlockParams) -> Block {
        let coinbase_params = TxParams {
            version: None,
            lock_time: None,
            input: None, 
            output: None,
        };        

        let coinbase = GenerateTx::valid_random(coinbase_params);

        match params.txs {
            Some(ref mut txs) => {
                txs.insert(0, coinbase);
            }
            None => {
                params.txs = Some(vec![coinbase]);
            }
        }
        Block::random(params)
    }
}