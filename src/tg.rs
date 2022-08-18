pub mod api {
    use crate::modbus_ats::ats_control::Ats;
    use crate::modbus_ats::ats_control::GeneratorLoad;
    use crate::modbus_winter_garden::winter_garden_control::WinterGarden;
    use error_chain::error_chain;
    use reqwest::StatusCode;

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
    fn send_notification(message: &str, chat_id: i32) {
        match send_message(message, chat_id) {
            Ok(_) => info!("send_message(message): ok"),
            Err(e) => info!("send_message(message) error: {}", e),
        }
    }

    fn send_from_start(chat_id: i32) {
        let message = "You have subscribed to the event notifications of the automatic ATS management system and the ATS monitoring application";
        send_notification(message, chat_id);
    }

    fn send_from_help(chat_id: i32) {
        let message = "/start - Subscribe to notifications of the automatic ATS management system and the ATS monitoring application%0A/help - Background information%0A/ats - ATS monitoring%0A/wintergarden - Winter Garden monitoring";
        send_notification(message, chat_id);
    }

    fn send_ats(chat_id: i32) {
        let ats: Ats = crate::psql::postgresql::select_ats().unwrap_or_default();

        let mains_power_supply = match ats.mains_power_supply {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let start_generator = match ats.start_generator {
            1 => "successful start",
            0 => "no start",
            _ => "error",
        };

        let generator_faulty = match ats.generator_faulty {
            1 => "faulty",
            0 => "no faulty",
            _ => "error",
        };

        let transmitted_work = match ats.transmitted_work {
            1 => "ok",
            0 => "no work",
            _ => "error",
        };

        let connection = match ats.connection {
            1 => "ok",
            0 => "no connection",
            _ => "error",
        };

        let generator_load: GeneratorLoad =
            crate::psql::postgresql::select_generator_load().unwrap_or_default();
        let ats_data = format!(
                    "Ats: %0Amains_power_supply: {} %0Astart_generator: {} %0Agenerator_faulty: {} %0Atransmitted_work: {} %0Aconnection: {} %0Agenerator_load: {}A",
                    mains_power_supply,
                    start_generator,
                    generator_faulty,
                    transmitted_work,
                    connection,
                    generator_load.load
                );
        send_notification(&ats_data, chat_id);
    }

    fn send_winter_garden(chat_id: i32) {
        let winter_garden: WinterGarden =
            crate::psql::postgresql::select_winter_garden().unwrap_or_default();

        let phyto_lighting_1 = match winter_garden.phyto_lighting_1 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let phyto_lighting_2 = match winter_garden.phyto_lighting_2 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let phyto_lighting_3 = match winter_garden.phyto_lighting_3 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let phyto_lighting_4 = match winter_garden.phyto_lighting_4 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let fan = match winter_garden.fan {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let automatic_watering_1 = match winter_garden.automatic_watering_1 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let automatic_watering_2 = match winter_garden.automatic_watering_2 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let automatic_watering_3 = match winter_garden.automatic_watering_3 {
            1 => "on",
            0 => "off",
            _ => "error",
        };

        let winter_garden_data = format!(
                    "Winter Garden: %0Aphyto_lighting_1: {} %0Aphyto_lighting_2: {} %0Aphyto_lighting_3: {} %0Aphyto_lighting_4: {} %0Afan: {} %0Aautomatic_watering_1: {} %0Aautomatic_watering_2: {} %0Aautomatic_watering_3: {} %0Atemperature_indoor: {}°C %0Ahumidity_indoor: {}% %0Aillumination_indoor: {} lx %0Aillumination_outdoor: {} lx",
                    phyto_lighting_1,
                    phyto_lighting_2,
                    phyto_lighting_3,
                    phyto_lighting_4,
                    fan,
                    automatic_watering_1,
                    automatic_watering_2,
                    automatic_watering_3,
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

    pub fn callback() {
        match crate::json::deserialize::last_message() {
            Ok((message, message_time, chat_id)) => {
                if message == "/start" {
                    let message_time_cache =
                        crate::psql::postgresql::select_message_time().unwrap_or_default();
                    info!(
                        "message = {}, message_time = {}, message_time_cache = {}",
                        message, message_time, message_time_cache
                    );
                    if message_time > message_time_cache {
                        send_from_start(chat_id);
                        match crate::psql::postgresql::insert_message_time(message_time) {
                            Ok(_) => info!("insert_message_time(message_time): ok"),
                            Err(e) => info!("insert_message_time(message_time) error: {}", e),
                        }
                    }
                }

                if message == "/help" {
                    let message_time_cache =
                        crate::psql::postgresql::select_message_time().unwrap_or_default();
                    info!(
                        "message = {}, message_time = {}, message_time_cache = {}",
                        message, message_time, message_time_cache
                    );
                    if message_time > message_time_cache {
                        send_from_help(chat_id);
                        match crate::psql::postgresql::insert_message_time(message_time) {
                            Ok(_) => info!("insert_message_time(message_time): ok"),
                            Err(e) => info!("insert_message_time(message_time) error: {}", e),
                        }
                    }
                }

                if message == "/ats" {
                    let message_time_cache =
                        crate::psql::postgresql::select_message_time().unwrap_or_default();
                    info!(
                        "message = {}, message_time = {}, message_time_cache = {}",
                        message, message_time, message_time_cache
                    );
                    if message_time > message_time_cache {
                        send_ats(chat_id);
                        match crate::psql::postgresql::insert_message_time(message_time) {
                            Ok(_) => info!("insert_message_time(message_time): ok"),
                            Err(e) => info!("insert_message_time(message_time) error: {}", e),
                        }
                    }
                }

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
