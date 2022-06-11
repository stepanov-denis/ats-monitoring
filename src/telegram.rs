pub mod bot {
    use teloxide::{prelude::*, utils::command::BotCommands};

    use std::error::Error;

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
        #[command(
            description = "Вот что делает этот бот:\nОтправляет уведомления о сбоях в работе АВР.\nОтправляет мгновенные значения систем мониторинга зимнего сада и АВР.\nДля получения списка команд введите команду: /help"
        )]
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
                bot.send_message(
                    message.chat.id,
                    format!("This is monitoring of winter garden."),
                )
                .await?
            }
            Command::Ats => {
                bot.send_message(message.chat.id, format!("This is monitoring ATS."))
                    .await?
            }
        };

        Ok(())
    }

    async fn send_alerts(
        bot: AutoSend<Bot>,
        msg: Message,
        s: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let bot = Bot::from_env().auto_send();
        bot.send_message(msg.chat.id, s).await?;
        Ok(())
    }
}
