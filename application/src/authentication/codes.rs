use rand::Rng;

pub fn generate_random_code(length: i8) -> String {
    let mut rng = rand::thread_rng();
    let code: String = (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    code
}