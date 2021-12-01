use chrono::prelude::*;
use sha2::{Sha256, Digest};

pub struct Block {
    index: usize,
    timestamp: DateTime<Utc>,
    data: Vec<u8>,
    hash: Vec<u8>,
    previous_hash: Vec<u8>,
}

pub struct Blockchain {
    chain: Vec<Block>,
}

fn hash_hex(hash: &Vec<u8>) -> String {
    let mut hex = String::from("");
    for byte in hash {
        hex = format!("{}{:02x}", hex, byte);
    }
    hex
}

fn compute_hash(index: usize, timestamp: DateTime<Utc>, data: &Vec<u8>, previous_hash: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();

    println!("### computing hash for index {}", index);

    // maybe we can do the same without format!() ?
    hasher.update(format!("{}", index));
    hasher.update(format!("{}", timestamp));
    hasher.update(format!("{:?}", data));
    hasher.update(format!("{:?}", previous_hash));
    let res = hasher.finalize();
    return res[..].to_vec();
}

impl Block {
    fn new(index: usize, timestamp: DateTime<Utc>, data: Vec<u8>, previous_hash: Vec<u8>) -> Self {
        let hash = compute_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            hash,
            previous_hash,
        }
    }

    fn compute_hash(self: &Self) -> Vec<u8> {
        compute_hash(self.index, self.timestamp, &self.data, &self.previous_hash)
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::new(0, Utc::now(), vec!(), vec!())],
        }
    }

    pub fn add(self: &mut Self, data: Vec<u8>) {
        let chain_len = self.chain.len();
        let latest_block = &self.chain[chain_len-1];
        let previous_hash = latest_block.hash.clone();
        let new_block = Block::new(latest_block.index+1, Utc::now(), data, previous_hash);
        self.chain.push(new_block);
    }

    pub fn is_valid(self: &Self) -> bool {
        let mut prev = &self.chain[0];
        for block in &self.chain[1..] {
            let bhash = block.compute_hash();
            if block.hash != bhash {
                println!("block {} has invalid hash {}", block.index, hash_hex(&bhash));
                return false;
            }

            if block.previous_hash != prev.hash {
                println!("block {} has different previous_hash", block.index);
                println!("     prev.hash           {}", hash_hex(&prev.hash));
                println!("     block.previous_hash {}", hash_hex(&block.previous_hash));
                return false;
            }

            prev = block;
        }

        true
    }

    // #[cfg(build = "debug")]
    pub fn dump_chain(self: &Self) {
        for block in &self.chain[0..] {
            println!("block: index         = {}", block.index);
            println!("       timestamp     = {}", block.timestamp);
            println!("       data          = {:?}", block.data);
            println!("       previous_hash = {}", hash_hex(&block.previous_hash));
            println!("       hash          = {}", hash_hex(&block.hash));
            println!();
        }

        println!("is_valid {}", self.is_valid());
    }

    // #[cfg(build = "debug")]
    pub fn tamper_data(self: &mut Self, index: usize, data: Vec<u8>) {
        self.chain[index].data = data;
    }
}
