use rand::Rng;
pub fn gen_ran() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen()
}
