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
        info!("executing an http request to an telegram bot api for update");
        let resp = reqwest::blocking::get(update_url())?;
        let text = resp.text();
        let result = text.unwrap_or_default();
        Ok(result)
    }

    /// Create url for telegram bot api, with sendMessage method.
    fn message_url(message: &str, chat_id: i32) -> String {
        let mut s = String::from("https://api.telegram.org/bot");
        s.push_str(&crate::read_env::env::read_str("TG_BOT_TOKEN").unwrap_or_default());
        s.push_str("/sendMessage?chat_id=");
        s.push_str(&chat_id.to_string());
        s.push_str("&text=");
        s.push_str(message);
        s
    }

    /// Sending SMS notification.
    fn send_message(message: &str, chat_id: i32) -> Result<()> {
        info!("executing an http request to an telegram bot api for send message");
        let resp = reqwest::blocking::get(message_url(message, chat_id))?;
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
    pub fn send_notification(message: &str, chat_id: i32) {
        match send_message(message, chat_id) {
            Ok(_) => info!("send_message(message): ok"),
            Err(e) => info!("send_message(message) error: {}", e),
        }
    }

    fn send_winter_garden(chat_id: i32) {
        let winter_garden: WinterGarden =
            crate::psql::postgresql::select_winter_garden().unwrap_or_default();
        let winter_garden_data = format!(
                    "Winter Garden: %0Aphyto_lighting_1: {} %0Aphyto_lighting_2: {} %0Aphyto_lighting_3: {} %0Aphyto_lighting_4: {} %0Afan: {} %0Aautomatic_watering_1: {} %0Aautomatic_watering_2: {} %0Aautomatic_watering_3: {} %0Atemperature_indoor: {} %0Ahumidity_indoor: {} %0Aillumination_indoor: {} %0Aillumination_outdoor: {}",
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
        send_notification(&winter_garden_data, chat_id);
    }

    pub fn send_alarm(message: &str) {
        let vec_chat_id = crate::psql::postgresql::select_chat_id().unwrap_or_default();
        for id in vec_chat_id {
            send_notification(message, id);
        }
    }

    pub fn callback_winter_garden() {
        match crate::json::deserialize::last_message() {
            Ok((message, message_time, chat_id)) => {
                if message == "/wintergarden" {
                    let message_time_cache =
                        crate::psql::postgresql::select_message_time().unwrap_or_default();
                    info!(
                        "message = {}, message_time = {}, message_time_cache = {}",
                        message, message_time, message_time_cache
                    );
                    if message_time > message_time_cache {
                        send_winter_garden(chat_id);
                        match crate::psql::postgresql::insert_message_time(message_time) {
                            Ok(_) => info!("insert_message_time(message_time): ok"),
                            Err(e) => info!("insert_message_time(message_time) error: {}", e),
                        }
                    }
                }
            }
            Err(e) => info!("callback_winter_garden() error: {}", e),
        }
    }

    pub fn update_chat_id() {
        match crate::json::deserialize::chat_id() {
            Ok(_) => info!("crate::json::deserialize::chat_id() ok"),
            Err(e) => info!("crate::json::deserialize::chat_id() error: {}", e),
        }
    }
}
