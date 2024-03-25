use rand::Rng;
pub fn get_random_id() -> u32 {
    let mut rng = rand::thread_rng();
    let n1 = rng.gen::<u32>();
    n1
}
pub fn _inc(mut x: u32) -> u32 {
    x = x + 1;
    x
}

