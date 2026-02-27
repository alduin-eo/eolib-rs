use rand::RngExt;

/// returns a random swap multiple
pub fn generate_swap_multiple() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(6..=12)
}
