pub mod generator {
    /// Logging event "Alarm! The generator is faulty! Urgently perform service work!".
    fn log_alarm() {
        let event = "Alarm! The generator is faulty! Urgently perform service work!";
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(event) {
            Ok(_) => info!("insert_event(): {}", event),
            Err(e) => info!("{}", e)
        }
    }

    /// Logging request for operation of the generator
    /// in the mode of transmission of electricity from the power grid.
    fn log_request_to_generator() {
        info!(
            "request for operation of the generator 
            in the mode of transmission of electricity from the power grid"
        );
        info!(
            "response from postgresql: generator_faulty = {:?}",
            crate::psql::postgresql::select_generator_faulty()
        );
    }

    /// Inner loop for cyclic polling of the emergency generator.
    fn inner_loop_generator_faulty() {
        'inner: loop {
            // Checking the connection of the app to the PLC.
            if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                log_request_to_generator();
                // Checking the faulty condition of the generator
                // 0 => the generator is working properly
                // 1 => the generator is faulty.
                match crate::psql::postgresql::select_generator_faulty() {
                    Ok(0) => {
                        let event = "the efficiency of the generator in the mode 
                        of transmission of electricity from the power grid has been restored";
                        info!("{}", event);
                        // Records event to the SQL table 'app_log'.
                        match crate::psql::postgresql::insert_event(event) {
                            Ok(_) => info!("insert_event(): {}", event),
                            Err(e) => info!("{}", e)
                        }
                        // Sending SMS notification.
                        match crate::sms::gateway::send_notification("SMS_GEN_WORK_RESTORED") {
                            Ok(_) => info!("send_sms('SMS_GEN_WORK_RESTORED'): ok"),
                            Err(e) => info!("{}", e),
                        }
                        break 'inner;
                    }
                    _ => log_alarm(),
                }
            }
        }
    }

    /// The function of determining the serviceability/malfunction
    /// of the generator and notifying about it by SMS using the gateway API.
    pub fn generator_state() {
        // Checking the connection of the app to the PLC.
        if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
            log_request_to_generator();
            // Checking the faulty condition of the generator
            // 0 => the generator is working properly
            // 1 => the generator is faulty.
            match crate::psql::postgresql::select_generator_faulty() {
                Ok(1) => {
                    log_alarm();
                    // Sending SMS notification.
                    match crate::sms::gateway::send_notification("SMS_GEN_WORK_ERR") {
                        Ok(_) => info!("send_sms('SMS_GEN_WORK_ERR'): ok"),
                        Err(e) => info!("{}", e),
                    }
                    // Entering the generator polling cycle when a fault is detected.
                    inner_loop_generator_faulty();
                }
                _ => {
                    let event = "generator is working properly in the mode of electricity transmission from the power grid";
                    info!("{}", event);
                    // Records event to the SQL table 'app_log'.
                    match crate::psql::postgresql::insert_event(event) {
                        Ok(_) => info!("insert_event(): {}", event),
                        Err(e) => info!("{}", e)
                    }
                }
            }
        }
    }
}
