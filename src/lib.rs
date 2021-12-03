use chrono::prelude::*;
// use rand::prelude::*;

// const DIFFICULTY:usize = 2;

mod hash;
use hash::Hash;

mod block;
use block::Block;

pub struct Blockchain {
    // rng: ThreadRng,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            // rng: rand::thread_rng(),
            chain: vec![Block::zero()],
        }
    }

    pub fn add(self: &mut Self, data: Vec<u8>) -> bool {
        let chain_len = self.chain.len();
        let latest_block = &self.chain[chain_len-1];
        let previous_hash = latest_block.hash().clone();
        let new_block = Block::new(latest_block.index()+1, Utc::now(), data, previous_hash);
        // if new_block.mine(&mut self.rng, DIFFICULTY) {
        //     self.chain.push(new_block);
        //     return true;
        // }
        // false

        self.chain.push(new_block);
        true
    }

    pub fn is_valid(self: &Self) -> bool {
        let mut prev = &self.chain[0];
        for block in &self.chain[1..] {
            let bhash = block.compute_hash();
            if block.hash() != bhash {
                println!("block {} has invalid hash {}", block.index(), bhash);
                return false;
            }

            if block.previous_hash() != prev.hash() {
                println!("block {} has different previous_hash", block.index());
                println!("     prev.hash           {}", prev.hash());
                println!("     block.previous_hash {}", block.previous_hash());
                return false;
            }

            prev = block;
        }

        true
    }

    // #[cfg(build = "debug")]
    pub fn dump_chain(self: &Self) {
        for block in &self.chain[0..] {
            println!("block: index         = {}", block.index());
            println!("       timestamp     = {}", block.timestamp());
            println!("       data          = {:?}", block.data());
            println!("       previous_hash = {}", block.previous_hash());
            println!("       hash          = {}", block.hash());
            println!();
        }

        println!("is_valid {}", self.is_valid());
    }

    // #[cfg(build = "debug")]
    pub fn tamper_data(self: &mut Self, index: usize, data: Vec<u8>) {
        self.chain[index].tamper_data(data);
    }
}
