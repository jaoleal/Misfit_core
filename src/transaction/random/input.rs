use bitcoin::{
    hashes::Hash, secp256k1::{All, Secp256k1}, sighash::{EcdsaSighashType, SighashCache},
    transaction::Version, Amount, OutPoint, PrivateKey, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, Txid, Witness
};
use secp256k1::rand::{self, Rng};

use super::{
    output::OutputParams,
    transaction::{RandomTransacion, TxParams},
    script::{ScriptTypes, ScriptInfo, RandomScript, ScriptParams},
};

pub struct InputParams {
    pub outpoint: Option<OutPoint>,
    pub script: Option<ScriptBuf>,
    pub sequence: Option<Sequence>,
    pub witness: Option<Witness>,
    pub script_params: Option<ScriptParams>,
}

pub struct InputInfo {
    pub txin: TxIn,
    pub script_type: ScriptTypes,
}

impl Default for InputParams {
    fn default() -> Self {
        InputParams {
            outpoint: None,
            script: None,
            sequence: None,
            witness: None,
            script_params: None,
        }
    }
}

pub trait RandomInput {
    fn random(params: InputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> InputInfo;
}

impl RandomInput for TxIn {
    fn random(params: InputParams, curve: &Secp256k1<All>, privatekey: &PrivateKey) -> InputInfo {
        let outpoint = params.outpoint.unwrap_or_else(|| {
            let txid = Transaction::random(
                TxParams {
                    version: None,
                    lock_time: None,
                    input: Some(InputParams {
                        outpoint: Some(OutPoint {
                            txid: Txid::all_zeros(),
                            vout: rand::thread_rng().gen::<u32>(),
                        }),
                        script: None,
                        sequence: None,
                        witness: None,
                        script_params: None,
                    }),
                    output: Some(OutputParams::default()),
                },
                curve,
                privatekey,
            )
            .transaction
            .compute_txid();

            OutPoint {
                txid,
                vout: rand::thread_rng().gen::<u32>(),
            }
        });

        let script_info = match params.script {
            Some(script) => ScriptInfo {
                script,
                script_type: ScriptTypes::P2WPKH, // changing for P2WPKH by default
            },
            None => ScriptBuf::random(
                params.script_params.unwrap_or(ScriptParams {
                    script_type: Some(ScriptTypes::P2WPKH)
                }),
                curve,
                privatekey,
            ),
        };

        let sequence = params
            .sequence
            .unwrap_or_else(|| Sequence::MAX);

        let witness = generate_signature_witness(
            &script_info,
            curve,
            privatekey,
            &outpoint,
            sequence,
        );

        InputInfo {
            txin: TxIn {
                previous_output: outpoint,
                script_sig: ScriptBuf::new(), // empty Scriptsig for P2WPKH
                sequence,
                witness,
            },
            script_type: script_info.script_type,
        }
    }
}

fn generate_signature_witness(
    script_info: &ScriptInfo,
    curve: &Secp256k1<All>,
    privatekey: &PrivateKey,
    outpoint: &OutPoint,
    sequence: Sequence,
) -> Witness {
    let temp_tx = Transaction {
        version: Version(2),
        lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: *outpoint,
            script_sig: ScriptBuf::new(),
            sequence,
            witness: Witness::default(),
        }],
        output: vec![],
    };

    let mut sighash_cache = SighashCache::new(&temp_tx);

    match script_info.script_type {
        ScriptTypes::P2WPKH => {
            // P2WPKH signature (já implementado)
            let sighash = sighash_cache
                .p2wpkh_signature_hash(
                    0,
                    &script_info.script,
                    Amount::from_sat(50_000),
                    EcdsaSighashType::All,
                )
                .expect("Falha ao gerar sighash");

            let signature = curve.sign_ecdsa(
                &bitcoin::secp256k1::Message::from_digest_slice(&sighash[..])
                    .expect("Sighash inválido"),
                &privatekey.inner,
            );

            let mut witness_stack = Witness::new();
            let mut sig_ser = signature.serialize_der().to_vec();
            sig_ser.push(EcdsaSighashType::All as u8);
            
            witness_stack.push(sig_ser);
            witness_stack.push(PublicKey::from_private_key(curve, privatekey).to_bytes().to_vec());

            witness_stack
        },
        ScriptTypes::P2WSH => {
            // P2WSH signature
            let sighash = sighash_cache
                .p2wsh_signature_hash(
                    0,
                    &script_info.script,
                    Amount::from_sat(50_000),
                    EcdsaSighashType::All,
                )
                .expect("Falha ao gerar sighash P2WSH");

            let signature = curve.sign_ecdsa(
                &bitcoin::secp256k1::Message::from_digest_slice(&sighash[..])
                    .expect("Sighash inválido"),
                &privatekey.inner,
            );

            let mut witness_stack = Witness::new();
            let mut sig_ser = signature.serialize_der().to_vec();
            sig_ser.push(EcdsaSighashType::All as u8);
            
            witness_stack.push(sig_ser);
            witness_stack.push(script_info.script.to_bytes());

            witness_stack
        },
        ScriptTypes::P2TR => {
            println!("Taproot singnature not implemented yet");
            Witness::default()
        },
        ScriptTypes::P2TWEAKEDTR => {
            println!("tweaked Taproot signature not implemented yet");
            Witness::default()
        },
        ScriptTypes::P2PK | ScriptTypes::P2PKH | ScriptTypes::P2SH => {
            Witness::default()
        }
    }
}