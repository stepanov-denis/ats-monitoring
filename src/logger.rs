pub mod log {
    /// Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
    pub fn record(event: &str) {
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(event) {
            Ok(_) => info!("insert_event() ok: {}", event),
            Err(e) => {
                let message = format!("insert_event() '{}' error: {}", event, e);
                info!("{}", message);
                // Sending telegram notification.
                crate::tg::api::send_notification(event);
            }
        }
    }

    /// Create event "app connection error to PLC".
    /// and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
    pub fn event_err_connect_to_plc(message: &str) {
        let event = format!(
            "error: there is no connection between the app and the plc, {}",
            message
        );
        record(&event);
    }
}
