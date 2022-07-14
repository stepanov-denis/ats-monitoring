pub mod env {
    use std::env;

    pub fn read_str(s: &str) -> Option<String> {
        match env::var(s) {
            Ok(val) => return Some(val),
            Err(e) => info!("couldn't interpret {s}: {e}")
        }
        None
    }

    pub fn read_u16(s: &str) -> Option<u16> {
        match env::var(s) {
            Ok(val) => {
                match val.parse::<u16>() {
                    Ok(val) => return Some(val),
                    Err(e) => info!("couldn't interpret {s}: {}", e)
                }
            }
            Err(e) => info!("couldn't interpret {s}: {e}"),
        }
        None
    }
}
