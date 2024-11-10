use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_random_alphanumeric_string(length: usize) -> String {
    let mut rng = thread_rng();

    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}
