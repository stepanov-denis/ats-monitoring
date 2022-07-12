pub mod sms_gateway {
    /// Returns the uri for the http post request
    /// for the sms gateway API to send an sms message.
    pub fn sms_message(s: &str) -> Option<String> {
        let mut message = crate::read_env::env::read_str("GATEWAY_STR_CONNECTION").unwrap_or_default();
        message.push_str(&crate::read_env::env::read_str(s).unwrap_or_default());
        Some(String::from(message))
    }
}
