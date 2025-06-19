use bitcoin::{
    ecdsa::Signature, hashes::Hash, secp256k1::{Message, Secp256k1}, sighash::{EcdsaSighashType, SighashCache}, NetworkKind, OutPoint, PrivateKey, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, Txid, Witness
};
use secp256k1::rand::{self, Rng};

use super::{
    script::{RandomScript, ScriptParams, ScriptTypes},
    transaction::{RandomTransacion, TxParams},
};

pub struct InputParams {
    pub outpoint: Option<OutPoint>,
    pub script: Option<(ScriptBuf, ScriptTypes)>,
    pub sequence: Option<Sequence>,
    pub witness: Option<Witness>,
    pub script_params: Option<ScriptParams>,
    pub private_key: Option<PrivateKey>,
}

impl Default for InputParams {
    fn default() -> Self {
        InputParams {
            outpoint: None,
            script: None,
            sequence: None,
            witness: None,
            script_params: None,
            private_key: None,
        }
    }
}

pub trait RandomInput {
    fn random(params: InputParams) -> TxIn;
}

impl RandomInput for TxIn {
    fn random(params: InputParams) -> TxIn {
        let mut random_tx_params = TxParams::default();
        let mut random_input_params = InputParams::default();

        random_input_params.outpoint = Some(OutPoint {
            txid: Txid::all_zeros(),
            vout: rand::thread_rng().gen::<u32>(),
        });

        random_tx_params.input = Some(random_input_params);

        let random_input_tx = Transaction::random(random_tx_params);

        let outpoint = params.outpoint.unwrap_or_else(|| OutPoint {
            txid: random_input_tx.compute_txid(),
            vout: rand::thread_rng().gen::<u32>(),
        });

        let private_key = params
            .private_key
            .unwrap_or_else(|| PrivateKey::generate(NetworkKind::Main));

        let (script_buf, script_type) = params.script.unwrap_or_else(|| {
            ScriptBuf::random(
                params.script_params.unwrap_or(ScriptParams {
                    script_type: None,
                    private_key: Some(private_key),
                }),
            )
        });

        let sequence = params
            .sequence
            .unwrap_or_else(|| Sequence(rand::thread_rng().gen::<u32>()));

        let witness = sign_witness(random_input_tx, outpoint, script_buf.clone(), &private_key);

        TxIn {
            previous_output: outpoint,
            script_sig: script_buf,
            sequence,
            witness,
        }
    }
}

fn sign_witness(
    tx: Transaction,
    outpoint: OutPoint,
    script_buf: ScriptBuf,
    privatekey: &PrivateKey,
) -> Witness {
    let amount = &tx.output.iter().map(|tx_out| tx_out.value).sum();

    let sighash = SighashCache::new(tx)
        .p2wpkh_signature_hash(outpoint.vout.try_into().unwrap(), &script_buf, *amount, EcdsaSighashType::All)
        .unwrap();

    let signature = Signature {
        signature: Secp256k1::new().sign_ecdsa(
            &Message::from_digest_slice(sighash.as_byte_array()).unwrap(),
            &privatekey.inner,
        ),
        sighash_type: EcdsaSighashType::All,
    };
    
    let pub_key = PublicKey::from_private_key(&Secp256k1::new(), privatekey);

    Witness::p2wpkh(&signature, &pub_key.inner)
}