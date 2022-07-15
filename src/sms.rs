pub mod gateway {
    use error_chain::error_chain;
    use reqwest::StatusCode;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Returns the uri for the http post request
    /// for the sms gateway API to send an sms message.
    fn sms_message(s: &str) -> Option<String> {
        let mut message =
            crate::read_env::env::read_str("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(&crate::read_env::env::read_str(s).unwrap_or_default());
        Some(String::from(message))
    }

    /// Logging event "server error the sms notification was not sent".
    fn log_sms_gateway_server_error(response: reqwest::blocking::Response) {
        let event = format!(
            "error: the sms notification was not sent, status http request: {}",
            response.status()
        );
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(&event) {
            Ok(_) => info!("insert_event(): {}", event),
            Err(e) => info!("{}", e),
        }
    }

    /// Sending SMS notification.
    pub fn send_notification(message_env: &str) -> Result<()> {
        info!("executing an http request to an sms notification service provider");
        let resp = reqwest::blocking::get(sms_message(message_env).unwrap_or_default())?;
        match resp.status() {
            StatusCode::OK => {
                let event = format!(
                    "http request completed successfully, an sms message was sent: {:?}",
                    crate::read_env::env::read_str(message_env)
                );
                info!("{}", event);
                // Records event to the SQL table 'app_log'.
                match crate::psql::postgresql::insert_event(&event) {
                    Ok(_) => info!("insert_event(): {}", event),
                    Err(e) => info!("{}", e),
                }
            }
            _ => log_sms_gateway_server_error(resp),
        }
        Ok(())
    }
}
