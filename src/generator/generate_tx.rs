use bitcoin::{
    consensus::encode,
    hashes::Hash,
    locktime::absolute,
    secp256k1::{rand, Message, Secp256k1, SecretKey},
    sighash::{EcdsaSighashType, SighashCache},
    transaction, Address, Amount, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

pub struct generate_tx{
}

impl generate_tx{
    //here i can call the other types of functions to create the other types of transaction,
    pub fn generate_simple_p2wpkh() -> (String, String) {
            const DUMMY_UTXO_AMOUNT: Amount = Amount::from_sat(20_000_000);
            const SPEND_AMOUNT: Amount = Amount::from_sat(5_000_000);
            const CHANGE_AMOUNT: Amount = Amount::from_sat(14_999_000);

            // Initialize cryptographic context
            let secp = Secp256k1::new();
            
            // Generate sender keys and address
            let sender_sk = SecretKey::new(&mut rand::thread_rng());
            let sender_pubkey = bitcoin::PublicKey::new(sender_sk.public_key(&secp));
            let sender_wpkh = sender_pubkey.wpubkey_hash().expect("Compressed key");
            let sender_script = ScriptBuf::new_p2wpkh(&sender_wpkh);
            let receiver_address = Address::p2pkh(&sender_pubkey, Network::Bitcoin);

            // Build unsigned transaction
            let mut tx = Transaction {
                version: transaction::Version::TWO,
                lock_time: absolute::LockTime::ZERO,
                input: vec![TxIn {
                    previous_output: OutPoint { txid: Txid::all_zeros(), vout: 0 },
                    script_sig: ScriptBuf::default(),
                    sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                    witness: Witness::default(),
                }],
                output: vec![
                    TxOut { 
                        value: SPEND_AMOUNT,
                        script_pubkey: receiver_address.script_pubkey(),
                    },
                    TxOut {
                        value: CHANGE_AMOUNT,
                        script_pubkey: sender_script.clone(),
                    },
                ],
            };
            // Generate signature
            let mut sighasher = SighashCache::new(&mut tx);
            let sighash = sighasher
                .p2wpkh_signature_hash(0, &sender_script, DUMMY_UTXO_AMOUNT, EcdsaSighashType::All)
                .expect("Sighash creation failed");

            let signature = secp.sign_ecdsa(&Message::from(sighash), &sender_sk);
            let signed_sig = bitcoin::ecdsa::Signature { 
                signature, 
                sighash_type: EcdsaSighashType::All 
            };

            // Apply witness
            let pk = sender_sk.public_key(&secp);    
            *sighasher.witness_mut(0).unwrap() = Witness::p2wpkh(&signed_sig, &pk);
            let signed_tx = sighasher.into_transaction();

            //println!("Structuctured TX 📝 : {:#?}", signed_tx);
            let raw_transaction = hex::encode(encode::serialize(&signed_tx));
            let txid = signed_tx.compute_txid();   
            
            return (raw_transaction.to_string() , txid.to_string())
}
 
}


