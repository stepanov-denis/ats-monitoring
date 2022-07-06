pub mod env {
    extern crate dotenv;
    use dotenv::dotenv;
    use std::env;

    pub fn read(s: &str) -> Option<String> {
        dotenv().ok();
        Some(env::var(s).unwrap_or_default())
    }
}