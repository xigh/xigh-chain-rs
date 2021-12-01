use chrono::prelude::*;
use sha2::{Sha256, Digest};
use rand::prelude::*;

const DIFFICULTY:usize = 2;

pub struct Block {
    index: usize,
    timestamp: DateTime<Utc>,
    data: Vec<u8>,
    hash: Vec<u8>,
    nonce: usize,
    previous_hash: Vec<u8>,
}

pub struct Blockchain {
    rng: ThreadRng,
    chain: Vec<Block>,
}

fn hash_hex(hash: &Vec<u8>) -> String {
    let mut hex = String::from("");
    for byte in hash {
        hex = format!("{}{:02x}", hex, byte);
    }
    hex
}

fn compute_hash(index: usize, timestamp: DateTime<Utc>, data: &Vec<u8>, previous_hash: &Vec<u8>, nonce: usize) -> Vec<u8> {
    let mut hasher = Sha256::new();

    // println!("### computing hash for index {}, nonce={}", index, nonce);

    // maybe we can do the same without format!() ?
    hasher.update(format!("{}", index));
    hasher.update(format!("{}", timestamp));
    hasher.update(format!("{:?}", data));
    hasher.update(format!("{:?}", previous_hash));
    hasher.update(format!("{}", nonce));
    let res = hasher.finalize();
    return res[..].to_vec();
}

impl Block {
    fn new(index: usize, timestamp: DateTime<Utc>, data: Vec<u8>, previous_hash: Vec<u8>) -> Self {
        let nonce = 0usize;
        let hash = compute_hash(index, timestamp, &data, &previous_hash, nonce);
        Block {
            index,
            timestamp,
            data,
            hash,
            nonce,
            previous_hash,
        }
    }

    fn compute_hash(self: &Self) -> Vec<u8> {
        compute_hash(self.index, self.timestamp, &self.data, &self.previous_hash, self.nonce)
    }

    fn mine(self: &mut Self, rng: &mut ThreadRng, difficulty: usize) -> bool {
        while !is_bytes_start_with_nth_zeros(&self.hash, difficulty) {
            self.nonce = rng.next_u64() as usize;
            self.hash = self.compute_hash();
        }
        true
    }
}

// todo: put this elsewhere
fn is_bytes_start_with_nth_zeros(bytes: &Vec<u8>, n: usize) -> bool {
    if n >= bytes.len() {
        return false
    }
    for i in 0..n {
        let byte = bytes[i];
        if byte != 0 {
            return false
        }
    }
    true
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            rng: rand::thread_rng(),
            chain: vec![Block::new(0, Utc::now(), vec!(), vec!())],
        }
    }

    pub fn add(self: &mut Self, data: Vec<u8>) -> bool {
        let chain_len = self.chain.len();
        let latest_block = &self.chain[chain_len-1];
        let previous_hash = latest_block.hash.clone();
        let mut new_block = Block::new(latest_block.index+1, Utc::now(), data, previous_hash);
        if new_block.mine(&mut self.rng, DIFFICULTY) {
            self.chain.push(new_block);
            return true;
        }
        false
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
