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
                let phyto_lighting_1 = "";
                let phyto_lighting_2 = "";
                let phyto_lighting_3 = "";
                let phyto_lighting_4 = "";
                let fan = "";
                let automatic_watering_1 ="";
                    
                let automatic_watering_2 ="";
                    
                let automatic_watering_3 ="";
                    
                let temperature_indoor ="";
                    
                let humidity_indoor = "";
                let illumination_indoor ="";
                    
                let illumination_outdoor ="";
                    
                let mut connection = "";
                connection = "Connection is ok";
                connection = "Err connecting server to PLC";
                connection = "Err connecting server to PostgreSQL";
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
                    phyto_lighting_1,
                    phyto_lighting_2,
                    phyto_lighting_3,
                    phyto_lighting_4,
                    fan,
                    automatic_watering_1,
                    automatic_watering_2,
                    automatic_watering_3,
                    temperature_indoor,
                    humidity_indoor,
                    illumination_indoor,
                    illumination_outdoor
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
