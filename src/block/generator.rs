use bitcoin::Block;
use super::random::block::{BlockParams, RandomBlock};
use bitcoin::{absolute::LockTime, transaction::Version, TxOut, ScriptBuf, Sequence, Witness, OutPoint};

use crate::transaction::{
        generator::GenerateTx,
        random::{output::OutputParams, input::InputParams, transaction::TxParams}, };

pub struct GenerateBlock {}

impl GenerateBlock {
    pub fn valid_random(mut params: BlockParams) -> Block {
        let mut input_params = InputParams::default();
        input_params.outpoint = Some(OutPoint::null());

        let mut coinbase_params = TxParams::default();
        coinbase_params.input = Some(input_params);

        let coinbase = GenerateTx::valid_random(coinbase_params);

        let mut txs = params.txs.take().unwrap_or_default();
        txs.insert(0, coinbase);
        params.txs = Some(txs);

        Block::random(params)
    }
}
