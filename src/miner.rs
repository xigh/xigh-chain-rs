trait Miner {
    fn mine(self: &mut Self, rng: &mut ThreadRng, difficulty: usize);
}