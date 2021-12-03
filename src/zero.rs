impl Miner for Block {
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
        return false;
    }
    for i in 0..n {
        let byte = bytes[i];
        if byte != 0 {
            return false;
        }
    }
    true
}
