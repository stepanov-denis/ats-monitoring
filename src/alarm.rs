pub mod info {
    /// Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
    pub fn log_alarm(event: &str) {
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(event) {
            Ok(_) => info!("insert_event(): {}", event),
            Err(e) => info!("{}", e)
        }
    }

    /// Create event "app connection error to PLC".
    /// and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
    pub fn event_err_connect_to_plc(message: &str) {
        let event = format!("error: there is no connection between the app and the plc, {}",
        message);
        crate::alarm::info::log_alarm(&event);
    }
}