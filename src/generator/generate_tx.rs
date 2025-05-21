use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    key::{Keypair, TweakedKeypair, TweakedPublicKey},
    secp256k1::{rand, Secp256k1},
    transaction::Version,
    Amount, NetworkKind, OutPoint, PrivateKey, PublicKey, ScriptBuf, ScriptHash, Sequence,
    Transaction, TxIn, TxOut, Txid, WScriptHash, Witness, XOnlyPublicKey,
};
use secp256k1::rand::Rng;

pub struct GenerateTx {}

pub struct InputParams {
    outpoint: Option<OutPoint>,
    script: Option<ScriptBuf>,
    sequence: Option<Sequence>,
    witness: Option<Witness>,
}

pub struct OutputParams {
    value: Option<Amount>,
    script_pubkey: Option<ScriptBuf>,
}

pub enum ScriptTypes {
    P2PK,
    P2PKH,
    P2SH,
    P2TR,
    P2TWEAKEDTR,
    P2WPKH,
    P2WSH,
}
pub struct ScriptParams {
    script_type: Option<ScriptTypes>,
}

pub struct TxParams {
    pub(crate) version: Option<Version>,
    pub(crate) lock_time: Option<LockTime>,
    // TODO: Input count
    pub(crate) input: Option<InputParams>,
    // TODO: Output count
    pub(crate) output: Option<OutputParams>,
}

impl GenerateTx {
    // TODO: Valid: Transaction will be valid, but random params
    // TODO: Invalid: Transaction will be invalid with invalid params (not random)

    // Return random valid version
    pub fn random_version() -> Version {
        // Random standard
        if rand::thread_rng().gen_bool(0.5) {
            if rand::thread_rng().gen_bool(0.5) {
                return Version::ONE;
            }
            return Version::TWO;
        }

        // Random non_standard
        Version::non_standard(rand::thread_rng().gen::<i32>())
    }

    // Return random valid locktime
    pub fn random_locktime() -> LockTime {
        let choice = rand::thread_rng().gen_range(0..5);

        match choice {
            0 => LockTime::from_consensus(rand::thread_rng().gen::<u32>()),
            1 => LockTime::from_height(rand::thread_rng().gen::<u32>()).unwrap_or(LockTime::ZERO),
            2 => {
                let hex_value = format!("{:X}", rand::thread_rng().gen::<u32>());
                LockTime::from_hex(&hex_value).unwrap_or(LockTime::ZERO)
            }
            3 => LockTime::from_time(rand::thread_rng().gen::<u32>()).unwrap_or(LockTime::ZERO),
            _ => LockTime::ZERO,
        }
    }

    // Return random valid input
    pub fn random_input(params: InputParams) -> TxIn {
        let outpoint = params.outpoint.unwrap_or_else(|| {
            // Create a random transaction for use as outpoint
            let tx_id = Self::random_tx(TxParams {
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
                }),
                output: Some(OutputParams {
                    value: None,
                    script_pubkey: Some(Self::random_script(ScriptParams { script_type: None })),
                }),
            })
            .compute_txid();

            return OutPoint {
                txid: tx_id,
                vout: rand::thread_rng().gen::<u32>(),
            };
        });
        let script = params.script.unwrap_or(ScriptBuf::default()); // TODO: When random, get script from outpoint
        let sequence = params
            .sequence
            .unwrap_or_else(|| Sequence(rand::thread_rng().gen::<u32>()));
        let witness = params.witness.unwrap_or(Witness::default());

        TxIn {
            previous_output: outpoint,
            script_sig: script,
            sequence: sequence,
            witness: witness,
        }
    }

    // Return random valid scriptpubkey
    pub fn random_script(params: ScriptParams) -> ScriptBuf {
        let script_type =
            params
                .script_type
                .unwrap_or_else(|| match rand::thread_rng().gen_range(0..6) {
                    0 => ScriptTypes::P2PK,
                    1 => ScriptTypes::P2PKH,
                    2 => ScriptTypes::P2SH,
                    3 => ScriptTypes::P2TR,
                    4 => ScriptTypes::P2TWEAKEDTR,
                    5 => ScriptTypes::P2WPKH,
                    _ => ScriptTypes::P2WSH,
                });

        match script_type {
            ScriptTypes::P2PK => ScriptBuf::new_p2pk(&PublicKey::from_private_key(
                &Secp256k1::new(),
                &PrivateKey::generate(NetworkKind::Main),
            )),
            ScriptTypes::P2PKH => ScriptBuf::new_p2pkh(
                &PublicKey::from_private_key(
                    &Secp256k1::new(),
                    &PrivateKey::generate(NetworkKind::Main),
                )
                .pubkey_hash(),
            ),
            ScriptTypes::P2SH => ScriptBuf::new_p2sh(&ScriptHash::all_zeros()),
            ScriptTypes::P2TR => ScriptBuf::new_p2tr(
                &Secp256k1::new(),
                XOnlyPublicKey::from_keypair(&Keypair::new(
                    &Secp256k1::new(),
                    &mut rand::thread_rng(),
                ))
                .0,
                None,
            ),
            ScriptTypes::P2TWEAKEDTR => ScriptBuf::new_p2tr_tweaked(
                TweakedPublicKey::from_keypair(TweakedKeypair::dangerous_assume_tweaked(
                    Keypair::new(&Secp256k1::new(), &mut rand::thread_rng()),
                )),
            ),
            ScriptTypes::P2WPKH => ScriptBuf::new_p2wpkh(
                &PublicKey::from_private_key(
                    &Secp256k1::new(),
                    &PrivateKey::generate(NetworkKind::Main),
                )
                .wpubkey_hash()
                .unwrap(),
            ),
            ScriptTypes::P2WSH => ScriptBuf::new_p2wsh(&WScriptHash::all_zeros()),
        }
    }

    // Return random valid output
    pub fn random_output(params: OutputParams) -> TxOut {
        // TODO: Fee estimator
        // TODO: Amount random value needs to be more than the sum of inputs and fee
        let amount = params
            .value
            .unwrap_or_else(|| Amount::from_sat(rand::thread_rng().gen::<u64>()));
        let script = params
            .script_pubkey
            .unwrap_or_else(|| Self::random_script(ScriptParams { script_type: None })); // TODO: Add script params into output params

        TxOut {
            value: amount,
            script_pubkey: script,
        }
    }

    // Return random valid transaction
    pub fn random_tx(params: TxParams) -> Transaction {
        let input_params = params.input.unwrap_or(InputParams {
            outpoint: None,
            script: None,
            sequence: None,
            witness: None,
        });
        let output_params = params.output.unwrap_or(OutputParams {
            value: None,
            script_pubkey: None,
        });

        Transaction {
            version: params.version.unwrap_or_else(|| Self::random_version()),
            lock_time: params.lock_time.unwrap_or_else(|| Self::random_locktime()),
            input: vec![Self::random_input(input_params)],
            output: vec![Self::random_output(output_params)],
        }
    }
}
