pub mod deserialize {
    use serde::{Deserialize, Serialize};
    use serde_json::Result;


    #[derive(Serialize, Deserialize, Debug)]
    struct Update {
        pub ok: bool,
        pub result: Vec<UpdateResult>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateResult {
        pub update_id: i32,
        pub message: Message,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub message_id: i32,
        pub from: MessageFrom,
        pub chat: Chat,
        pub date: i32,
        pub text: String,
        pub entities: Vec<Entities>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MessageFrom {
        pub id: i32,
        pub is_bot: bool,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub language_code: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Chat {
        pub id: i32,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub r#type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Entities {
        pub offset: i32,
        pub length: i32,
        pub r#type: String,
    }

    pub fn last_message() -> Result<(String, i32)> {
        let data = crate::tg::api::update().unwrap_or_default();
        let format_data = format!(r#"{}"#, data);
        let update: Update = serde_json::from_str(&format_data)?;
        let len = update.result.len();
        if len > 0 {
            let message = &update.result[len-1].message.text;
            let message_time = &update.result[len-1].message.date;
            return Ok((message.to_string(), *message_time))
        }
        Ok(("empty string".to_string(), 0))
    }
}