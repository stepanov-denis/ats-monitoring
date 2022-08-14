pub mod api {
    use error_chain::error_chain;
    use reqwest::StatusCode;

    use crate::modbus_winter_garden::winter_garden_control::WinterGarden;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Create url for telegram bot api, with getUpdates method.
    fn update_url() -> String {
        let mut s = String::from("https://api.telegram.org/bot");
        s.push_str(&crate::read_env::env::read_str("TG_BOT_TOKEN").unwrap_or_default());
        s.push_str("/getUpdates");
        s
    }

    /// Sending teleagram-bot update.
    pub fn update() -> Result<String> {
        println!("executing an http request to an telegram bot api");
        let resp = reqwest::blocking::get(update_url())?;
        let text = resp.text();
        let result = text.unwrap_or_default();      
        Ok(result)
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

    fn send_winter_garden() {
                let winter_garden: WinterGarden = crate::psql::postgresql::select_winter_garden().unwrap_or_default();
                let winter_garden_data = format!(
                    "Winter Garden:\n
                    phyto_lighting_1: {}\n
                    phyto_lighting_2: {}\n
                    phyto_lighting_3: {}\n
                    phyto_lighting_4: {}\n
                    fan: {}\n
                    automatic_watering_1: {}\n
                    automatic_watering_2: {}\n
                    automatic_watering_3: {}\n
                    temperature_indoor: {}\n
                    humidity_indoor: {}\n
                    illumination_indoor: {}\n
                    illumination_outdoor: {}\n
                ",
                    winter_garden.phyto_lighting_1,
                    winter_garden.phyto_lighting_2,
                    winter_garden.phyto_lighting_3,
                    winter_garden.phyto_lighting_4,
                    winter_garden.fan,
                    winter_garden.automatic_watering_1,
                    winter_garden.automatic_watering_2,
                    winter_garden.automatic_watering_3,
                    winter_garden.temperature_indoor,
                    winter_garden.humidity_indoor,
                    winter_garden.illumination_indoor,
                    winter_garden.illumination_outdoor
                );
                send_notification(&winter_garden_data);
    }

    pub fn callback_winter_garden() {
        match crate::json::deserialize::last_message() {
            Ok((message, message_time)) => {
                if message == "/wintergarden" {
                    let message_time_cash = crate::psql::postgresql::select_message_time().unwrap_or_default();
                    if message_time > message_time_cash {
                        info!("message_time = {}, message_time_cash = {}", message_time, message_time_cash);
                        info!("сейчас отправлю...");
                        send_winter_garden();
                        info!("отправил");
                        info!("сейчас запишу в постгрес");
                        crate::psql::postgresql::insert_message_time(message_time);
                        info!("записал");
                    }
                }
            }
            Err(e) => info!("callback_winter_garden() error: {}", e)
        }
    }
}
