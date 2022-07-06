pub mod gateway {

    pub fn sms_message(s: &str) -> Option<String> {
        let mut message = crate::read_env::env::read("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(&crate::read_env::env::read(s).unwrap_or_default());
        Some(String::from(message))
    }
}

pub mod tg_bot {
    pub fn _tg_message(s: &str) -> Option<String> {
        Some(String::from(s))
    }
}
