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

    pub fn last_message() -> Result<(String, i32, i32)> {
        let data = crate::tg::api::update().unwrap_or_default();

        // r#""# required according to the serde_json documentation:
        // Some JSON input data as a &str. Maybe this comes from the user.
        // let data = r#"
        // {
        //     "name": "John Doe",
        //     "age": 43,
        //     "phones": [
        //         "+44 1234567",
        //         "+44 2345678"
        //     ]
        // }"#;
        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        // let p: Person = serde_json::from_str(data)?;

        // So clippy thinks it's useless_format:
        // There is no point of doing that. format!("foo") can be replaced by "foo".to_owned()
        // if you really need a String. The even worse &format!("foo") is often encountered in the wild.
        // format!("{}", foo) can be replaced by foo.clone() if foo: String or foo.to_owned() if foo: &str.

        #[allow(clippy::useless_format)]
        let format_data = format!(r#"{}"#, data);
        let update: Update = serde_json::from_str(&format_data)?;
        let len = update.result.len();
        if len > 0 {
            let message = &update.result[len - 1].message.text;
            let message_time = update.result[len - 1].message.date;
            let chat_id = update.result[len - 1].message.chat.id;
            return Ok((message.to_string(), message_time, chat_id));
        }
        Ok(("empty string".to_string(), 0, 0))
    }

    pub fn chat_id() -> Result<()> {
        let data = crate::tg::api::update().unwrap_or_default();

        // r#""# required according to the serde_json documentation:
        // Some JSON input data as a &str. Maybe this comes from the user.
        // let data = r#"
        // {
        //     "name": "John Doe",
        //     "age": 43,
        //     "phones": [
        //         "+44 1234567",
        //         "+44 2345678"
        //     ]
        // }"#;
        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        // let p: Person = serde_json::from_str(data)?;

        // So clippy thinks it's useless_format:
        // There is no point of doing that. format!("foo") can be replaced by "foo".to_owned()
        // if you really need a String. The even worse &format!("foo") is often encountered in the wild.
        // format!("{}", foo) can be replaced by foo.clone() if foo: String or foo.to_owned() if foo: &str.

        #[allow(clippy::useless_format)]
        let format_data = format!(r#"{}"#, data);
        let update: Update = serde_json::from_str(&format_data)?;
        let len = update.result.len();
        if len > 0 {
            let mut contains = false;
            let chat_id = update.result[len - 1].message.chat.id;
            let mut vec_chat_id = crate::psql::postgresql::select_chat_id().unwrap_or_default();
            for id in &vec_chat_id {
                if id == &chat_id {
                    contains = true;
                }
            }
            if !contains {
                vec_chat_id.push(chat_id);
                match crate::psql::postgresql::insert_chat_id(vec_chat_id) {
                    Ok(_) => info!("insert_chat_id(vec_chat_id): ok"),
                    Err(e) => info!("insert_chat_id(vec_chat_id) error: {}", e),
                }
            }
        }
        Ok(())
    }
}
