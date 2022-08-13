pub mod api {
    use error_chain::error_chain;
    use reqwest::StatusCode;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Create url for telegram bot api, with sendMessage method.
    fn message_url(message: &str) -> String {
        let mut s = String::from("https://api.telegram.org/bot");
        s.push_str(&crate::read_env::env::read_str("TG_BOT_TOKEN").unwrap_or_default());
        s.push_str("/sendMessage?chat_id=");
        s.push_str(&crate::read_env::env::read_str("CHAT_ID").unwrap_or_default());
        s.push_str("&text=");
        s.push_str(message);
        s
    }

    /// Sending SMS notification.
    fn send_message(message: &str) -> Result<()> {
        info!("executing an http request to an telegram bot api");
        let resp = reqwest::blocking::get(message_url(message))?;
        match resp.status() {
            StatusCode::OK => {
                let event = format!(
                    "http request completed successfully, an telegram message was sent: {}",
                    message
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
            _ => {
                let event = format!(
                    "error: the telegram notification was not sent, status http request: {}",
                    resp.status()
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
        }
        Ok(())
    }

    /// Match send_message(message: &str).
    pub fn send_notification(message: &str) {
        match send_message(message) {
            Ok(_) => {
                let event = format!(
                    "send_notification(): ok, the message has been sent: {}",
                    message
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
            Err(e) => {
                let event = format!("send_notification(): error, the telegram notification was not sent, status http request: {}", e);
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
        }
    }
}
