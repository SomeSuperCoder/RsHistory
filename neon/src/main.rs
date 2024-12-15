use bip39::Mnemonic;
use crypto::KeyPair;
use miniz_oxide::deflate::compress_to_vec;
use neon::{block::Block, history::generate_slot};
use std::fs;

fn main() {
    let seed_phrase = fs::read_to_string("seed_phrase.txt").unwrap();
    let mut keypair = KeyPair::from_mnemonic(&Mnemonic::parse(seed_phrase).unwrap());

    let block = Block::new(generate_slot("UeckerAllen-MichaelJames".to_string()), 0, &mut keypair);
    // let block_as_vec = serde_json::to_vec(&block).unwrap();
    // let compressed_block = compress_to_vec(block_as_vec.as_slice(), 6);

    // let borshed_block = borsh::to_vec(&block).unwrap();
    // let compressd_borshed_block = compress_to_vec(borshed_block.as_slice(), 6);

    fs::write("genesis_block.bin", serde_json::to_string(&block).unwrap()).unwrap();
}
