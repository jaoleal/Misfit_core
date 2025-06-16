use super::{
    input::{InputParams, RandomInput},
    locktime::RandomLockTime,
    output::{OutputParams, RandomOutput},
    version::RandomVersion,
};
use bitcoin::{absolute::LockTime, key::PrivateKey, secp256k1::{All, Secp256k1}, transaction::Version, Transaction, TxIn, TxOut};

pub struct TxParams {
    pub(crate) version: Option<Version>,
    pub(crate) lock_time: Option<LockTime>,
    pub(crate) input: Option<InputParams>,
    pub(crate) output: Option<OutputParams>,
}

impl Default for TxParams {
    fn default() -> Self {
        TxParams {
            version: None,
            lock_time: None,
            input: None,
            output: None,
        }
    }
}

pub trait RandomTransacion {
    fn random(params: TxParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> Transaction;
}

impl RandomTransacion for Transaction {
    fn random(params: TxParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> Transaction {
        let input_params = params.input.unwrap_or_default();
        let output_params = params.output.unwrap_or_default();

        Transaction {
            version: params.version.unwrap_or_else(|| Version::random()),
            lock_time: params.lock_time.unwrap_or_else(|| LockTime::random()),
            input: vec![TxIn::random(input_params, curve, privatekey)],
            output: vec![TxOut::random(output_params, curve, privatekey)],
        }
    }
}
