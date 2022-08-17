pub mod gateway {
    use error_chain::error_chain;
    use reqwest::StatusCode;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Returns the URL for the http post request
    /// for the sms gateway API to send an sms message.
    fn sms_message(s: &str) -> Option<String> {
        let mut message =
            crate::read_env::env::read_str("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(&crate::read_env::env::read_str(s).unwrap_or_default());
        Some(message)
    }

    /// Sending SMS notification.
    fn send_message(message_env: &str) -> Result<()> {
        info!("executing an http request to an sms notification service provider");
        let resp = reqwest::blocking::get(sms_message(message_env).unwrap_or_default())?;
        match resp.status() {
            StatusCode::OK => {
                let event = format!(
                    "http request completed successfully, an sms message was sent: {:?}",
                    crate::read_env::env::read_str(message_env)
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
            _ => {
                let event = format!(
                    "error: the sms notification was not sent, status http request: {}",
                    resp.status()
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
            }
        }
        Ok(())
    }

    /// Sending telegram notification.
    pub fn send_notification(message_env: &str) {
        match send_message(message_env) {
            Ok(_) => {
                info!("send_notification('{}'): ok", message_env);
            }
            Err(e) => {
                let event = format!(
                    "send_notification(
                    '{}',
                ) error: {}",
                    message_env, e
                );
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Sending telegram notification.
                crate::tg::api::send_alarm(&event);
            }
        }
    }
}
