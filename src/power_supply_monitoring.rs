pub mod power_supply {
    extern crate chrono;
    extern crate timer;
    use std::sync::mpsc::channel;

    /// Standby timer to confirm the power off from the mains.
    fn timer_for_delay(sec: i64) {
        let timer = timer::Timer::new();
        let (tx, rx) = channel();

        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(sec), move || {
            tx.send(()).unwrap();
            let _ignored = tx.send(());
        });

        rx.recv().unwrap();
    }

    /// Logging a request for a power failure in the power grid.
    fn log_request_to_mains_power_supply() {
        info!("request for power from the mains");
        info!(
            "response from postgresql: mains_power_supply = {:?}",
            crate::psql::postgresql::select_mains_power_supply()
        );
    }

    /// Logging event: "power from the power grid has been restored".
    fn power_restored() {
        let event = "power from the power grid has been restored";
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(event) {
            Ok(_) => info!("insert_event(): {}", event),
            Err(e) => info!("{}", e),
        }
    }

    /// ATS polling cycle after power outage.
    fn inner_loop() {
        'inner: loop {
            // Reading the value of the "connection" variable from the TRIM5 PLC via Modbus TCP
            // to check the connection of the app to the PLC.
            if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                // Logging a request for a power failure in the power grid.
                log_request_to_mains_power_supply();
                // Getting the mains_power_supply value
                // 0 - there is no power supply from the mains
                // 1 - there is power from the mains.
                match crate::psql::postgresql::select_mains_power_supply() {
                    Ok(1) => {
                        // Logging event: "power from the power grid has been restored".
                        power_restored();
                        // Getting the generator_work value
                        // 0 - mains power is transmitted via ATS
                        // 1- mains power is not transmitted via ATS.
                        match crate::psql::postgresql::select_transmitted_work() {
                            Ok(1) => {
                                let event = "the power supply from the power grid has been restored, the generator is working fine";
                                info!("{}", event);
                                // Records event to the SQL table 'app_log'.
                                match crate::psql::postgresql::insert_event(event) {
                                    Ok(_) => info!("insert_event(): {}", event),
                                    Err(e) => info!("{}", e),
                                }
                                // Sending SMS notification.
                                match crate::sms::gateway::send_notification(
                                    "SMS_POW_RESTORED_GEN_OK",
                                ) {
                                    Ok(_) => {
                                        info!("send_notification('SMS_POW_RESTORED_GEN_OK'): ok")
                                    }
                                    Err(e) => info!("{}", e),
                                }
                            }
                            Ok(0) => {
                                let event = "power from the power grid has been restored, the generator is faulty";
                                info!("{}", event);
                                // Records event to the SQL table 'app_log'.
                                match crate::psql::postgresql::insert_event(event) {
                                    Ok(_) => info!("insert_event(): {}", event),
                                    Err(e) => info!("{}", e),
                                }
                                // Sending SMS notification.
                                match crate::sms::gateway::send_notification(
                                    "SMS_POW_RESTORED_GEN_ERR",
                                ) {
                                    Ok(_) => {
                                        info!("send_notification('SMS_POW_RESTORED_GEN_ERR'): ok")
                                    }
                                    Err(e) => info!("{}", e),
                                }
                            }
                            Err(e) => info!("{}", e),
                            _ => info!("the generator_work value is not 0 or 1"),
                        }
                        break 'inner;
                    }
                    Ok(0) => {
                        let event = "the power from the power grid has not been restored yet, after the shutdown";
                        info!("{}", event);
                        // Records event to the SQL table 'app_log'.
                        match crate::psql::postgresql::insert_event(event) {
                            Ok(_) => info!("insert_event(): {}", event),
                            Err(e) => info!("{}", e),
                        }
                    }
                    Err(e) => info!("{}", e),
                    _ => info!("the mains_power_supply value is not 0 or 1"),
                }
            }
        }
    }

    /// Detection of a power failure from the mains,
    /// restoration of power from the mains,
    /// successful start of the generator,
    /// generator start failure,
    /// operability/malfunction of the power supply transmission mode from the mains by the generator.
    pub fn ats_monitoring() {
        // Checking the connection of the app to the PLC.
        if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
            // Logging a request for a power failure in the power grid.
            log_request_to_mains_power_supply();
            // Getting the mains_power_supply value
            // 0 - there is no power supply from the mains
            // 1 - there is power from the mains.
            match crate::psql::postgresql::select_mains_power_supply() {
                Ok(0) => {
                    let delay = 90;
                    let event = format!(
                        "there was a power failure from the power grid,
                    waiting {} seconds for confirmation of the absence of power from the mains",
                        delay
                    );
                    info!("{}", event);
                    // Records event to the SQL table 'app_log'.
                    match crate::psql::postgresql::insert_event(&event) {
                        Ok(_) => info!("insert_event(): {}", event),
                        Err(e) => info!("{}", e),
                    }
                    // Standby timer to confirm the power off from the mains.
                    timer_for_delay(delay);
                    // Checking the connection of the app to the PLC.
                    if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                        // Request for the availability of power from the mains and request the start status of the generator.
                        log_request_to_mains_power_supply();
                        // Getting the mains_power_supply value
                        // 0 - there is no power supply from the mains
                        // 1 - there is power from the mains.
                        match crate::psql::postgresql::select_mains_power_supply() {
                            Ok(0) => {
                                let event = "confirmation of the absence of mains power";
                                info!("{}", event);
                                // Records event to the SQL table 'app_log'.
                                match crate::psql::postgresql::insert_event(&event) {
                                    Ok(_) => info!("insert_event(): {}", event),
                                    Err(e) => info!("{}", e),
                                }
                                // Getting the start_generator value
                                // 0 - generator startup failure
                                // 1 - successful generator startup
                                match crate::psql::postgresql::select_start_generator() {
                                    Ok(1) => {
                                        let event = "disconnecting power from the mains, successful start of the generator";
                                        info!("{}", event);
                                        // Records event to the SQL table 'app_log'.
                                        match crate::psql::postgresql::insert_event(&event) {
                                            Ok(_) => info!("insert_event(): {}", event),
                                            Err(e) => info!("{}", e),
                                        }
                                        // Sending SMS notification.
                                        match crate::sms::gateway::send_notification(
                                            "SMS_START_GEN_OK",
                                        ) {
                                            Ok(_) => {
                                                info!("send_notification('SMS_START_GEN_OK'): ok")
                                            }
                                            Err(e) => info!("{}", e),
                                        }
                                    }
                                    Ok(0) => {
                                        let event = "disconnecting power from the mains, the generator startup failed";
                                        info!("{}", event);
                                        // Records event to the SQL table 'app_log'.
                                        match crate::psql::postgresql::insert_event(&event) {
                                            Ok(_) => info!("insert_event(): {}", event),
                                            Err(e) => info!("{}", e),
                                        }
                                        // Sending SMS notification.
                                        match crate::sms::gateway::send_notification(
                                            "SMS_START_GEN_ERR",
                                        ) {
                                            Ok(_) => {
                                                info!("send_notification('SMS_START_GEN_ERR'): ok")
                                            }
                                            Err(e) => info!("{}", e),
                                        }
                                    }
                                    Err(e) => info!("{}", e),
                                    _ => info!("the start_generator value is not 0 or 1"),
                                }
                                // ATS polling cycle after power outage.
                                inner_loop();
                            }
                            Ok(1) => {
                                // Logging event: "power from the power grid has been restored".
                                power_restored();
                            }
                            Err(e) => info!("{}", e),
                            _ => info!("the mains_power_supply value is not 0 or 1"),
                        }
                    }
                }
                Ok(1) => {
                    let event = "the power is supplied from the mains";
                    info!("{}", event);
                    // Records event to the SQL table 'app_log'.
                    match crate::psql::postgresql::insert_event(&event) {
                        Ok(_) => info!("insert_event(): {}", event),
                        Err(e) => info!("{}", e),
                    }
                }
                Err(e) => info!("{}", e),
                _ => info!("the mains_power_supply value is not 0 or 1"),
            }
        }
    }
}
