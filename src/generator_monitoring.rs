pub mod generator {
    /// Create a request event for the operation of the generator
    /// in the mode of transmission of electricity from the power grid.
    fn log_request_to_generator() -> String {
        format!("request for operation of the generator
        in the mode of transmission of electricity from the power grid\nresponse from postgresql: generator_faulty = {:?}",
        crate::psql::postgresql::select_generator_faulty())
    }

    /// Inner loop for cyclic polling of the emergency generator.
    fn inner_loop_generator_faulty() {
        'inner: loop {
            // Checking the connection of the app to the PLC.
            if crate::modbus_ats::ats_control::reading_connection() == Some(true) {
                // Create a request event for the operation of the generator
                // in the mode of transmission of electricity from the power grid.
                let event = log_request_to_generator();
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Cheking generator_faulty value
                // 0 - generator is working properly in the mode of electricity transmission from the power grid
                // 1 - the generator does not work in the mode of transmission of electricity from the power grid
                // 2 - the generator_faulty value is not 0 or 1.
                match crate::psql::postgresql::select_generator_faulty() {
                    Ok(0) => {
                        let event = "the efficiency of the generator in the mode 
                        of transmission of electricity from the power grid has been restored";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                        // Sending telegram notification.
                        crate::tg::api::send_alarm(event);
                        // Sending SMS notification.
                        crate::sms::gateway::send_notification("SMS_GEN_WORK_RESTORED");
                        break 'inner;
                    }
                    Ok(1) => {
                        let event =
                            "Alarm! The generator is faulty! Urgently perform service work!";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                    }
                    Ok(2) => {
                        let event = "the generator_faulty value is not 0 or 1";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                        // Sending telegram notification.
                        crate::tg::api::send_alarm(event);
                    }
                    Err(e) => {
                        let event = format!("generator_faulty() error: {}", e);
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                        // Sending telegram notification.
                        crate::tg::api::send_alarm(&event);
                    }
                    _ => {
                        let event = "error: the generator_faulty value is _";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                        // Sending telegram notification.
                        crate::tg::api::send_alarm(event);
                    }
                }
            }
        }
    }

    /// The function of determining the serviceability/malfunction
    /// of the generator and notifying about it by SMS using the gateway API.
    pub fn generator_monitoring() {
        // Checking the connection of the app to the PLC.
        if crate::modbus_ats::ats_control::reading_connection() == Some(true) {
            // Create a request event for the operation of the generator
            // in the mode of transmission of electricity from the power grid.
            let event = log_request_to_generator();
            // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
            crate::logger::log::record(&event);
            // Checking generator_faulty value
            // 0 - generator is working properly in the mode of electricity transmission from the power grid
            // 1 - the generator does not work in the mode of transmission of electricity from the power grid
            // 2 - the generator_faulty value is not 0 or 1.
            match crate::psql::postgresql::select_generator_faulty() {
                Ok(0) => {
                    let event = "generator is working properly in the mode of electricity transmission from the power grid";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(event);
                }
                Ok(1) => {
                    let event = "Alarm! The generator is faulty! Urgently perform service work!";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(event);
                    // Sending telegram notification.
                    crate::tg::api::send_alarm(event);
                    // Sending SMS notification.
                    crate::sms::gateway::send_notification("SMS_GEN_WORK_ERR");
                    // Entering the generator polling cycle when a fault is detected.
                    inner_loop_generator_faulty();
                }
                Ok(2) => {
                    let event = "the generator_faulty value is not 0 or 1";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(event);
                    // Sending telegram notification.
                    crate::tg::api::send_alarm(event);
                }
                Err(e) => {
                    let event = format!("generator_faulty() error: {}", e);
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                    // Sending telegram notification.
                    crate::tg::api::send_alarm(&event);
                }
                _ => {
                    let event = "error: the generator_faulty value is _";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(event);
                    // Sending telegram notification.
                    crate::tg::api::send_alarm(event);
                }
            }
        }
    }
}
