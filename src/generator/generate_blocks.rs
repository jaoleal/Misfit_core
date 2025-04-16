use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use sha2::{Digest, Sha256};
use hex;

#[derive(Debug)]
pub struct GenerateBlock {
    pub _version: u32,
    pub _prev_block_hash: String,
    pub _merkle_root: String,
    pub _timestamp: u32,
    pub _bits: u32,
    pub _nonce: u32,
    pub _transaction_count: u32,
    pub _block_header: String,
}

impl GenerateBlock {
    pub fn new(txids: Vec<String>) -> Self {
        let prev_block_hash = generate_random_bitcoin_block_hash();
        let merkle_root = merkleroot(txids.clone());
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
        let bits:i32 = 0x1d00ffff; // Example value (Bitcoin's genesis block bits)
        let nonce:i32 = 0;
        let transaction_count = txids.len() as u32;

        let version_bytes = 1u32.to_le_bytes();
        let mut prev_block_bytes = hex::decode(&prev_block_hash).unwrap();
        prev_block_bytes.reverse(); // Convert to original byte order
        let merkle_root_bytes = hex::decode(&merkle_root).unwrap();
        let timestamp_bytes = timestamp.to_le_bytes();
        let bits_bytes = bits.to_le_bytes();
        let nonce_bytes = nonce.to_le_bytes();

        let mut header_bytes = Vec::new();
        header_bytes.extend_from_slice(&version_bytes);
        header_bytes.extend_from_slice(&prev_block_bytes);
        header_bytes.extend_from_slice(&merkle_root_bytes);
        header_bytes.extend_from_slice(&timestamp_bytes);
        header_bytes.extend_from_slice(&bits_bytes);
        header_bytes.extend_from_slice(&nonce_bytes);

        let block_header = hex::encode(header_bytes);

        GenerateBlock {
            _version: 1,
            _prev_block_hash: prev_block_hash,
            _merkle_root: to_little_endian(&merkle_root),
            _timestamp: timestamp,
            _bits: bits.try_into().unwrap(),
            _nonce: nonce.try_into().unwrap(),
            _transaction_count:transaction_count,
            _block_header:block_header,
        }
    }
}

fn to_little_endian(hex: &str) -> String {
    hex.chars().collect::<Vec<char>>()
        .chunks_exact(2)
        .rev()
        .flat_map(|chunk| chunk.iter())
        .collect()
}

fn hash256(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    let first = Sha256::digest(&bytes);
    let second = Sha256::digest(first);
    hex::encode(second)
}

fn merkleroot(txids: Vec<String>) -> String {
    let txids_le: Vec<String> = txids.into_iter()
        .map(|hex| to_little_endian(&hex))
        .collect();
    compute_merkle_root(txids_le)
}

fn compute_merkle_root(mut hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return String::from("0000000000000000000000000000000000000000000000000000000000000000");
    }
    if hashes.len() == 1 {
        return hashes[0].clone();
    }
    if hashes.len() % 2 != 0 {
        let last = hashes.last().unwrap().clone();
        hashes.push(last);
    }
    let mut next_level = Vec::new();
    for i in (0..hashes.len()).step_by(2) {
        let pair = format!("{}{}", hashes[i], hashes[i+1]);
        next_level.push(hash256(&pair));
    }
    compute_merkle_root(next_level)
}

fn generate_random_bitcoin_block_hash() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    hex::encode(bytes) // Original byte order
}