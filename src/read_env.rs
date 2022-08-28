pub mod env {
    use std::env;

    /// Reading environment variables of the string type from the config.toml file.
    pub fn read_str(s: &str) -> Option<String> {
        match env::var(s) {
            Ok(val) => return Some(val),
            Err(e) => {
                let event: String = format!("read_str() error: couldn't interpret {s}: {e}");
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Sending telegram notification.
                crate::tg::api::send_alarm(&event);
            }
        }
        None
    }

    /// Reading environment variables of type u16 from config.toml file.
    pub fn read_u16(s: &str) -> Option<u16> {
        match env::var(s) {
            Ok(val) => match val.parse::<u16>() {
                Ok(val) => return Some(val),
                Err(e) => {
                    let event: String =
                        format!("val.parse::<u16>() error: couldn't interpret {s}: {e}");
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                    // Sending telegram notification.
                    crate::tg::api::send_alarm(&event);
                }
            },
            Err(e) => {
                let event: String = format!("read_u16() error: couldn't interpret {s}: {e}");
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Sending telegram notification.
                crate::tg::api::send_alarm(&event);
            }
        }
        None
    }
}
