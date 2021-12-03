use chrono::prelude::*;
use sha2::{Sha256, Digest};

use crate::Hash;

fn compute_hash(index: usize, timestamp: DateTime<Utc>, data: &Vec<u8>, previous_hash: &Hash) -> Hash { //, nonce: usize) -> Hash {
    let mut hasher = Sha256::new();

    // maybe we can do the same, faster, without format!() ?
    hasher.update(format!("{}", index));
    hasher.update(format!("{}", timestamp));
    hasher.update(format!("{:?}", data));
    hasher.update(format!("{}", previous_hash));
    // hasher.update(format!("{}", nonce));
    let res = hasher.finalize();
    Hash::from(res[..].to_vec())
}

pub struct Block {
    index: usize,
    timestamp: DateTime<Utc>,
    data: Vec<u8>,
    hash: Hash,
    previous_hash: Hash,
    // required by mining algorithm
    // nonce: usize,
}

impl Block {
    pub fn new(index: usize, timestamp: DateTime<Utc>, data: Vec<u8>, previous_hash: Hash) -> Self {
        // let nonce = 0usize;
        let hash = compute_hash(index, timestamp, &data, &previous_hash); // , nonce);
        Block {
            index,
            timestamp,
            data,
            hash,
            // nonce,
            previous_hash,
        }
    }

    pub fn compute_hash(self: &Self) -> Hash {
        compute_hash(self.index, self.timestamp, &self.data, &self.previous_hash) // , self.nonce)
    }

    pub fn index(self: &Self) -> usize {
        self.index
    }

    pub fn timestamp(self: &Self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn data(self: &Self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn hash(self: &Self) -> Hash {
        self.hash.clone()
    }

    pub fn previous_hash(self: &Self) -> Hash {
        self.previous_hash.clone()
    }

    pub fn tamper_data(self: &mut Self, data: Vec<u8>) {
        self.data = data;
    }
}
