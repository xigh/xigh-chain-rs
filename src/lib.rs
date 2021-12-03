use chrono::prelude::*;
use sha2::{Sha256, Digest};
use rand::prelude::*;

const DIFFICULTY:usize = 2;

pub struct Hash {
    bytes: Vec<u8>
}

impl Hash {
    fn new() -> Hash {
        return Hash {
            bytes: vec!(),
        }
    }
}

impl std::cmp::PartialEq for Hash {
    fn eq(self: &Self, s2: &Self) -> bool {
        self.bytes == s2.bytes
    }
}

impl std::fmt::Display for Hash {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            if let Err(err) = write!(f, "{:02x}", byte) {
                return Err(err);
            }
        }
        Ok(())
    }
}

impl std::clone::Clone for Hash {
    fn clone(self: &Self) -> Self {
        return Hash {
            bytes: self.bytes.clone()
        }
    }
}

pub struct Block {
    index: usize,
    timestamp: DateTime<Utc>,
    data: Vec<u8>,
    hash: Hash,
    previous_hash: Hash,
    // required by mining algorithm
    nonce: usize,
}

pub struct Blockchain {
    rng: ThreadRng,
    chain: Vec<Block>,
}

fn compute_hash(index: usize, timestamp: DateTime<Utc>, data: &Vec<u8>, previous_hash: &Hash, nonce: usize) -> Hash {
    let mut hasher = Sha256::new();

    // maybe we can do the same, faster, without format!() ?
    hasher.update(format!("{}", index));
    hasher.update(format!("{}", timestamp));
    hasher.update(format!("{:?}", data));
    hasher.update(format!("{}", previous_hash));
    hasher.update(format!("{}", nonce));
    let res = hasher.finalize();
    return Hash{
        bytes: res[..].to_vec(),
    };
}

impl Block {
    fn new(index: usize, timestamp: DateTime<Utc>, data: Vec<u8>, previous_hash: Hash) -> Self {
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

    fn compute_hash(self: &Self) -> Hash {
        compute_hash(self.index, self.timestamp, &self.data, &self.previous_hash, self.nonce)
    }

    fn mine(self: &mut Self, rng: &mut ThreadRng, difficulty: usize) -> bool {
        while !do_bytes_start_with_nth_zeros(&self.hash.bytes, difficulty) {
            self.nonce = rng.next_u64() as usize;
            self.hash = self.compute_hash();
        }
        true
    }
}

// todo: put this elsewhere
fn do_bytes_start_with_nth_zeros(bytes: &Vec<u8>, n: usize) -> bool {
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
            chain: vec![Block::new(0, Utc::now(), vec!(), Hash::new())],
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
                println!("block {} has invalid hash {}", block.index, block.hash);
                return false;
            }

            if block.previous_hash != prev.hash {
                println!("block {} has different previous_hash", block.index);
                println!("     prev.hash           {}", prev.hash);
                println!("     block.previous_hash {}", block.previous_hash);
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
            println!("       previous_hash = {}", block.previous_hash);
            println!("       hash          = {}", block.hash);
            println!();
        }

        println!("is_valid {}", self.is_valid());
    }

    // #[cfg(build = "debug")]
    pub fn tamper_data(self: &mut Self, index: usize, data: Vec<u8>) {
        self.chain[index].data = data;
    }
}
