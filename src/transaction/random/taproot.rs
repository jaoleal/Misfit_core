use bitcoin::{
    key::{Keypair, TapTweak},//TweakedKeypair
    hashes::Hash,
    Amount,
    secp256k1::{Secp256k1, Message},
    sighash::{Prevouts, SighashCache, TapSighashType},
    Transaction, TxOut, Witness, 
};
use crate::transaction::random::script::ScriptTypes;
use secp256k1::rand::{self};


pub struct TaprootWitnessParams {
    pub transaction: Option<Transaction>,
    pub vout: Option<usize>,
    pub utxo: Option<TxOut>,
    pub keypair: Option<Keypair>,
    pub script_type: Option<ScriptTypes>,
}

impl Default for TaprootWitnessParams {
    fn default() -> Self {
        TaprootWitnessParams {
            transaction: None,
            vout: None,
            utxo: None,
            keypair: None,
            script_type: Some(ScriptTypes::P2TR), 
        }
    }
}

pub trait RandomTaprootWitness {
    fn random(params: TaprootWitnessParams) -> Witness;
}

impl RandomTaprootWitness for Witness {
    fn random(mut _params: TaprootWitnessParams) -> Witness {
        let secp = Secp256k1::new();

        let keypair = _params.keypair.unwrap_or_else(|| {
            let sk = bitcoin::secp256k1::SecretKey::new(&mut rand::thread_rng());
            Keypair::from_secret_key(&secp, &sk)
        });

        let tx = _params.transaction.unwrap_or_else(|| {
            Transaction {
                version: bitcoin::transaction::Version::TWO,
                lock_time: bitcoin::absolute::LockTime::ZERO,
                input: vec![bitcoin::TxIn::default()],
                output: vec![],
            }
        });

        let vout = _params.vout.unwrap_or(0);
        let utxo = _params.utxo.unwrap_or_else(|| TxOut {
            value: Amount::from_sat(100_000),
            script_pubkey: bitcoin::ScriptBuf::default(),
        });
        let binding = [utxo.clone()];
        let prevouts = Prevouts::All(&binding);
        let mut cache = SighashCache::new(&tx);
        let sighash = cache
            .taproot_key_spend_signature_hash(
                vout,
                &prevouts,
                TapSighashType::Default,
            )
            .expect("sighash");

        let tweaked = keypair.tap_tweak(&secp, None);
        let msg = Message::from_digest(sighash.to_byte_array());
        let schnorr_sig = secp.sign_schnorr(&msg, &tweaked.to_keypair());

        let signature = bitcoin::taproot::Signature {
            signature: schnorr_sig,
            sighash_type: TapSighashType::Default,
        };

        Witness::p2tr_key_spend(&signature)
    }
}