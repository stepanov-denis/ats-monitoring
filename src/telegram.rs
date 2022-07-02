pub mod bot {
    
    use std::error::Error;
    use teloxide::{prelude::*, utils::command::BotCommands};

    #[tokio::main]
    pub async fn bot_commands() {
        let bot = Bot::from_env().auto_send();

        teloxide::commands_repl(bot, answer, Command::ty()).await;
    }

    #[derive(BotCommands, Clone)]
    #[command(
        rename = "lowercase",
        description = "Описание команд управления ботом:"
    )]
    enum Command {
        #[command(description = "Подписаться на уведомления событий АВР.")]
        Start,
        #[command(description = "Справочная информация.")]
        Help,
        #[command(description = "Мониторинг зимнего сада.")]
        WinterGarden,
        #[command(description = "Мониторинг АВР.")]
        Ats,
    }

    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match command {
            Command::Start => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Help => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::WinterGarden => {
                let phyto_lighting_1 = crate::skydb::skytable::get_i32_skydb("phyto_lighting_1");
                let phyto_lighting_2 = crate::skydb::skytable::get_i32_skydb("phyto_lighting_2");
                let phyto_lighting_3 = crate::skydb::skytable::get_i32_skydb("phyto_lighting_3");
                let phyto_lighting_4 = crate::skydb::skytable::get_i32_skydb("phyto_lighting_4");
                let fan = crate::skydb::skytable::get_i32_skydb("fan");
                let automatic_watering_1 =
                    crate::skydb::skytable::get_i32_skydb("automatic_watering_1");
                let automatic_watering_2 =
                    crate::skydb::skytable::get_i32_skydb("automatic_watering_2");
                let automatic_watering_3 =
                    crate::skydb::skytable::get_i32_skydb("automatic_watering_3");
                let temperature_indoor =
                    crate::skydb::skytable::get_i32_skydb("temperature_indoor");
                let humidity_indoor = crate::skydb::skytable::get_i32_skydb("humidity_indoor");
                let illumination_indoor =
                    crate::skydb::skytable::get_i32_skydb("illumination_indoor");
                let illumination_outdoor =
                    crate::skydb::skytable::get_i32_skydb("illumination_outdoor");
                let mut connection = "".to_string();
                if crate::skydb::skytable::unix_sql() + 5.00
                    >= crate::skydb::skytable::unix_sql_now()
                {
                    if crate::skydb::skytable::plc_connect() == 1 {
                        connection = "Connection is ok".to_string();
                    } else {
                        connection = "Err connecting server to PLC".to_string();
                    }
                } else {
                    connection = "Err connecting server to PostgreSQL".to_string();
                }
                let winter_garden_data = format!(
                    "Winter Garden:\n
                {}\n
                > phyto_lighting_1: {}\n
                > phyto_lighting_2: {}\n
                > phyto_lighting_3: {}\n
                > phyto_lighting_4: {}\n
                > fan: {}\n
                > automatic_watering_1: {}\n
                > automatic_watering_2: {}\n
                > automatic_watering_3: {}\n
                > temperature_indoor: {}\n
                > humidity_indoor: {}\n
                > illumination_indoor: {}\n
                > illumination_outdoor: {}\n
                ",
                    connection,
                    phyto_lighting_1.unwrap(),
                    phyto_lighting_2.unwrap(),
                    phyto_lighting_3.unwrap(),
                    phyto_lighting_4.unwrap(),
                    fan.unwrap(),
                    automatic_watering_1.unwrap(),
                    automatic_watering_2.unwrap(),
                    automatic_watering_3.unwrap(),
                    temperature_indoor.unwrap(),
                    humidity_indoor.unwrap(),
                    illumination_indoor.unwrap(),
                    illumination_outdoor.unwrap()
                );
                bot.send_message(message.chat.id, winter_garden_data)
                    .await?
            }
            Command::Ats => {
                bot.send_message(message.chat.id, "This is monitoring ATS.".to_string())
                    .await?
            }
        };

        Ok(())
    }

    async fn send_alerts(
        _bot: AutoSend<Bot>,
        msg: Message,
        s: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let bot = Bot::from_env().auto_send();
        bot.send_message(msg.chat.id, s).await?;
        Ok(())
    }
}
