// src/main.rs
use hex;
mod block;

fn main() {
   // exemple_header_str = "020000004c6adace8008af29c20745473c9025d583379d3922eca6270000000000000000c9a79656029fd65b3cc804eea935daa6430e4b72aa4c96170f3ec79d6f3eeb87147ed55266660219ed711859";
   // let block_header = hex::encode("020000004c6adace8008af29c20745473c9025d583379d3922eca6270000000000000000c9a79656029fd65b3cc804eea935daa6430e4b72aa4c96170f3ec79d6f3eeb87147ed55266660219ed711859");
  let block_header = block::BlockHeader::new();

    println!("Constructed Block Header:");
   //  println!("The block header: {}", block_header);
    println!("Version: {}", block_header.version);
    println!("Previous Block Hash: {:?}", block_header.prev_block_hash);
    println!("Merkle Root: {:?}", block_header.merkle_root);
    println!("Timestamp: {}", block_header.timestamp);
    println!("Difficulty Target: {}", block_header.difficulty_target);
    println!("Nonce: {}", block_header.nonce);

}