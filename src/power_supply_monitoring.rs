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
    fn log_request_to_mains_power_supply() -> String {
        format!(
            "request for power from the mains\nresponse from postgresql: mains_power_supply = {:?}",
            crate::psql::postgresql::select_mains_power_supply()
        )
    }

    /// Logging event: "power from the power grid has been restored".
    fn power_restored() {
        let event = "power from the power grid has been restored";
        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
        crate::logger::log::record(&event);
    }

    /// ATS polling cycle after power outage.
    fn inner_loop() {
        'inner: loop {
            // Reading the value of the "connection" variable from the TRIM5 PLC via Modbus TCP
            // to check the connection of the app to the PLC.
            if crate::modbus_ats::ats_control::reading_connection() == Some(true) {
                // Logging a request for a power failure in the power grid.
                let event = log_request_to_mains_power_supply();
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Checking the mains_power_supply value
                // 0 - there is no power supply from the city power grid
                // 1 - there is power from the city power grid
                // 2 - the mains_power_supply value is not 0 or 1.
                match crate::psql::postgresql::select_mains_power_supply() {
                    Ok(1) => {
                        // Logging event: "power from the power grid has been restored".
                        power_restored();
                        // Checking the transmitted_work value
                        // 0 - mains power is transmitted via ATS
                        // 1- mains power is not transmitted via ATS.
                        // 2 - the transmitted_work value is not 0 or 1.
                        match crate::psql::postgresql::select_transmitted_work() {
                            Ok(1) => {
                                let event = "the power supply from the power grid has been restored, the generator is working fine";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                                // Sending SMS notification.
                                match crate::sms::gateway::send_notification(
                                    "SMS_POW_RESTORED_GEN_OK",
                                ) {
                                    Ok(_) => {
                                        info!("send_notification('SMS_POW_RESTORED_GEN_OK'): ok")
                                    }
                                    Err(e) => {
                                        let event = format!(
                                            "send_notification(
                                            'SMS_POW_RESTORED_GEN_OK',
                                        ) error: {}",
                                            e
                                        );
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                    }
                                }
                            }
                            Ok(0) => {
                                let event = "the power supply has not been restored, the generator is faulty";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                                // Sending SMS notification.
                                match crate::sms::gateway::send_notification(
                                    "SMS_POW_RESTORED_GEN_ERR",
                                ) {
                                    Ok(_) => {
                                        info!("send_notification('SMS_POW_RESTORED_GEN_ERR'): ok")
                                    }
                                    Err(e) => {
                                        let event = format!(
                                            "send_notification(
                                            'SMS_POW_RESTORED_GEN_ERR',
                                        ) error: {}",
                                            e
                                        );
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                    }
                                }
                            }
                            Ok(2) => {
                                let event = "the transmitted_work value is not 0 or 1";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(event);
                            }
                            Err(e) => {
                                let event = format!("select_transmitted_work() error: {}", e);
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                            }
                            _ => {
                                let event = "the transmitted_work value is _";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(event);
                            }
                        }
                        break 'inner;
                    }
                    Ok(0) => {
                        let event = "the power from the power grid has not been restored yet, after the shutdown";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                    }
                    Ok(2) => {
                        let event = "the mains_power_supply value is not 0 or 1";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                    }
                    Err(e) => {
                        let event = format!("select_mains_power_supply() error: {}", e);
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                    }
                    _ => {
                        let event = "error: the mains_power_supply value is _";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                    }
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
        if crate::modbus_ats::ats_control::reading_connection() == Some(true) {
            // Logging a request for a power failure in the power grid.
            let event = log_request_to_mains_power_supply();
            // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
            crate::logger::log::record(&event);
            // Checking the mains_power_supply value
            // 0 - there is no power supply from the city power grid
            // 1 - there is power from the city power grid
            // 2 - the mains_power_supply value is not 0 or 1.
            match crate::psql::postgresql::select_mains_power_supply() {
                Ok(0) => {
                    let delay = 90;
                    let event = format!(
                        "there was a power failure from the power grid,
                    waiting {} seconds for confirmation of the absence of power from the mains",
                        delay
                    );
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                    // Standby timer to confirm the power off from the mains.
                    timer_for_delay(delay);
                    // Checking the connection of the app to the PLC.
                    if crate::modbus_ats::ats_control::reading_connection() == Some(true) {
                        // Request for the availability of power from the mains and request the start status of the generator.
                        let event = log_request_to_mains_power_supply();
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                        // Checking the mains_power_supply value
                        // 0 - there is no power supply from the city power grid
                        // 1 - there is power from the city power grid
                        // 2 - the mains_power_supply value is not 0 or 1.
                        match crate::psql::postgresql::select_mains_power_supply() {
                            Ok(0) => {
                                let event = "confirmation of the absence of mains power";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                                // Checking the start_generator value
                                // 0 - generator start failure
                                // 1 - the generator has started
                                // 2 - the start_generator value is not 0 or 1.
                                match crate::psql::postgresql::select_start_generator() {
                                    Ok(1) => {
                                        let event = "disconnecting power from the mains, successful start of the generator";
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                        // Sending SMS notification.
                                        match crate::sms::gateway::send_notification(
                                            "SMS_START_GEN_OK",
                                        ) {
                                            Ok(_) => {
                                                info!("send_notification('SMS_START_GEN_OK'): ok")
                                            }
                                            Err(e) => {
                                                let event = format!(
                                                    "send_notification(
                                                    'SMS_START_GEN_OK',
                                                ) error: {}",
                                                    e
                                                );
                                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                                crate::logger::log::record(&event);
                                            }
                                        }
                                    }
                                    Ok(0) => {
                                        let event = "disconnecting power from the mains, the generator startup failed";
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                        // Sending SMS notification.
                                        match crate::sms::gateway::send_notification(
                                            "SMS_START_GEN_ERR",
                                        ) {
                                            Ok(_) => {
                                                info!("send_notification('SMS_START_GEN_ERR'): ok")
                                            }
                                            Err(e) => {
                                                let event = format!(
                                                    "send_notification(
                                                    'SMS_START_GEN_ERR',
                                                ) error: {}",
                                                    e
                                                );
                                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                                crate::logger::log::record(&event);
                                            }
                                        }
                                    }
                                    Ok(2) => {
                                        let event =
                                            format!("the start_generator() value is not 0 or 1");
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                    }
                                    Err(e) => {
                                        let event =
                                            format!("select_start_generator() error: {}", e);
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                    }
                                    _ => {
                                        let event =
                                            format!("error: the start_generator value is _");
                                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                        crate::logger::log::record(&event);
                                    }
                                }
                                // ATS polling cycle after power outage.
                                inner_loop();
                            }
                            Ok(1) => {
                                // Logging event: "power from the power grid has been restored".
                                power_restored();
                            }
                            Ok(2) => {
                                let event = format!("the mains_power_supply value is not 0 or 1");
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                            }
                            Err(e) => {
                                let event = format!("select_mains_power_supply() error: {}", e);
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                            }
                            _ => {
                                let event = "error: the mains_power_supply value is _";
                                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                                crate::logger::log::record(&event);
                            }
                        }
                    }
                }
                Ok(1) => {
                    let event = "the power is supplied from the mains";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                }
                Ok(2) => {
                    let event = format!("the mains_power_supply value is not 0 or 1");
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                }
                Err(e) => {
                    let event = format!("select_mains_power_supply() error: {}", e);
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                }
                _ => {
                    let event = "error: the mains_power_supply value is _";
                    // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                    crate::logger::log::record(&event);
                }
            }
        }
    }
}
