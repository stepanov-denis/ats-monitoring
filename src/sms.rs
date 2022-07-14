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
        let mut message = crate::read_env::env::read_str("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(&crate::read_env::env::read_str(s).unwrap_or_default());
        Some(String::from(message))
    }

    /// Logging event "server error the sms notification was not sent".
    fn log_sms_gateway_server_error(response: reqwest::blocking::Response) {
        info!("status http request: {}", response.status());
        info!("server error the sms notification was not sent");
        // Records log
        // "Server error! Ошибка! SMS уведомление не было отправлено!"
        // in the sql table "журнал_работы_приложения".
        match crate::psql::postgresql::log_server_err() {
            Ok(_) => info!("crate::psql::postgresql::log_server_err(): ok"),
            Err(e) => info!("{}", e)
        }
    }

    /// Sending SMS.
    pub fn send_notification(message_env: &str) -> Result<()> {
        info!("executing an http request to an sms notification service provider");
        let resp = reqwest::blocking::get(
            sms_message(message_env).unwrap_or_default(),
        )?;
        match resp.status() {
            StatusCode::OK => {
                info!("http request completed successfully");
                info!(
                    "an sms message was sent: {:?}",
                    crate::read_env::env::read_str(message_env)
                );
                // Records log 
                // "Отправлено SMS сообщение:
                // Работоспособность генератора в режиме трансляции питания 
                // от электросети восстановлена.
                // in the sql table "журнал_работы_приложения".
                match crate::psql::postgresql::log_send_sms_generator_work_restored(message_env) {
                    Ok(_) => info!("crate::psql::postgresql::log_send_sms_generator_work_restored(message_env): ok"),
                    Err(e) => info!("{}", e)
                }
            }
            _ => log_sms_gateway_server_error(resp)
        }
        Ok(())
    }
}