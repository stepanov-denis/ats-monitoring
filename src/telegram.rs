pub mod bot {

    use std::error::Error;
    use teloxide::{prelude::*, utils::command::BotCommands};
    use crate::modbus_winter_garden::winter_garden_control::WinterGarden;
    use postgres::{Client, Error as PostgresError, NoTls};

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
        #[command(description = "Начало работы.")]
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
                // select_winter_garden().await?;
                // let winter_garden_data = format!(
                //     "Winter Garden:\n
                //     phyto_lighting_1: {}\n
                //     phyto_lighting_2: {}\n
                //     phyto_lighting_3: {}\n
                //     phyto_lighting_4: {}\n
                //     fan: {}\n
                //     automatic_watering_1: {}\n
                //     automatic_watering_2: {}\n
                //     automatic_watering_3: {}\n
                //     temperature_indoor: {}\n
                //     humidity_indoor: {}\n
                //     illumination_indoor: {}\n
                //     illumination_outdoor: {}\n
                // ",
                //     winter_garden.phyto_lighting_1,
                //     winter_garden.phyto_lighting_2,
                //     winter_garden.phyto_lighting_3,
                //     winter_garden.phyto_lighting_4,
                //     winter_garden.fan,
                //     winter_garden.automatic_watering_1,
                //     winter_garden.automatic_watering_2,
                //     winter_garden.automatic_watering_3,
                //     winter_garden.temperature_indoor,
                //     winter_garden.humidity_indoor,
                //     winter_garden.illumination_indoor,
                //     winter_garden.illumination_outdoor
                // );
                bot.send_message(message.chat.id, "This is monitoring Winter Garden")
                    .await?
            }
            Command::Ats => {
                bot.send_message(message.chat.id, "This is monitoring ATS.")
                    .await?
            }
        };

        Ok(())
    }
}
