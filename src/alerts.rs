pub mod gateway {

    /// SMS Gateway string connection
    pub fn sms_gateway_string_connection(s: &str) -> Option<String> {
        Some(String::from(s))
    }

    pub fn sms_message(s: &str) -> Option<String> {
        let mut message =
            sms_gateway_string_connection("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(s);
        Some(message)
    }

    pub fn _tg_message(s: &str) -> Option<String> {
        Some(String::from(s))
    }
}
