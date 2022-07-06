pub mod env {
    use std::env;

    pub fn read(s: &str) -> Option<String> {
        Some(env::var(s).unwrap())
    }
}
