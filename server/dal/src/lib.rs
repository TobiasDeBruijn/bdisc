use rand::Rng;

pub mod mysql;
pub mod fs;

fn generate_string(len: usize) -> String {
    rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(len).map(char::from).collect()
}