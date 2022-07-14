pub mod power_supply {
    extern crate chrono;
    extern crate timer;
    use std::sync::mpsc::channel;
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Timer for delay 'inner: loop.
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
        info!(
            "request for power from the mains"
        );
        info!(
            "response from PostgreSQL: mains_power_supply = {:?}",
            crate::psql::postgresql::select_mains_power_supply()
        );
    }

    fn inner_loop() {
        'inner: loop {
            if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                log_request_to_mains_power_supply();
                match crate::psql::postgresql::select_mains_power_supply() {
                    Ok(1) => {
                        info!("power from the power grid has been restored");
                        crate::psql::postgresql::log_power_restored();
                        match crate::psql::postgresql::select_generator_work() {
                            Ok(1) => {
                                info!("Генератор исправен. Генератор работает");
                                crate::psql::postgresql::event_power_restored_generator_work_ok();
                                crate::psql::postgresql::log_power_restored_generator_ok();
                                crate::sms::gateway::send_notification("SMS_POW_RESTORED_GEN_OK");
                            }
                            Ok(0) => {
                                info!("Генератор неисправен. Генератор не работает");
                                crate::psql::postgresql::event_power_restored_generator_work_err();
                                crate::psql::postgresql::log_power_restored_generator_err();
                                crate::sms::gateway::send_notification("SMS_POW_RESTORED_GEN_ERR");
                            }
                            Err(e) => info!("{}", e),
                            _ => info!("some error")
                        }
                        break 'inner;
                    }
                    Ok(0) => {
                        info!("Питание от электросети еще не было восстановлено, после отключения");
                        crate::psql::postgresql::log_power_dont_restored();
                    }
                    Err(e) => info!("{}", e),
                    _ => info!("some error")
                }
            }
        }
    }

    /// Main spawn - the function for detecting a power failure from the mains/restoring power from the mains,
    /// successful start of the generator, failure of the generator start, and notifications about these events.
    /// Additional spawn - the function of determining the serviceability/malfunction of the generator
    /// and notifying about it by SMS using the gateway API.
    pub fn ats_state() -> Result<()> {
        // Checking the connection of the app to the PLC.
        if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
            log_request_to_mains_power_supply();
            match crate::psql::postgresql::select_mains_power_supply() {
                Ok(0) => {
                    info!("Произошел сбой питания от электросети");
                    info!("Ожидание (90 секунд) подтверждения отсутствия питания от электросети");
                    crate::psql::postgresql::log_power_failure();
                    timer_for_delay(90);
                    if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                        // Request for the availability of power from the mains and request the start status of the generator.
                        log_request_to_mains_power_supply();
                        match crate::psql::postgresql::select_mains_power_supply() {
                            Ok(0) => {
                                info!("Подтверждение отсутствия питания от электросети");
                                crate::psql::postgresql::log_power_failure_confirmed();
                                match crate::psql::postgresql::select_start_generator() {
                                    Ok(1) => {
                                        info!("Успешный старт генератора");
                                        crate::psql::postgresql::event_power_failure_start_generator_ok();
                                        crate::psql::postgresql::log_start_generator_ok();
                                        crate::sms::gateway::send_notification("SMS_START_GEN_OK");
                                    }
                                    Ok(0) => {
                                        info!("Сбой старта генератора!");
                                        crate::psql::postgresql::event_power_failure_start_generator_err();
                                        crate::psql::postgresql::log_start_generator_err();
                                        crate::sms::gateway::send_notification("SMS_START_GEN_ERR");
                                    }
                                    Err(e) => info!("{}", e),
                                    _ => info!("some error")
                                }
                            }
                            Ok(1) => {
                                info!("Питание от электросети восстановлено");
                                crate::psql::postgresql::log_power_restored();
                            }
                            Err(e) => info!("{}", e),
                            _ => info!("some error")
                        }
                    }
                }
                Ok(1) => {
                    info!("Питание от электросети есть");
                    crate::psql::postgresql::event_power_ok();
                    crate::psql::postgresql::log_power_ok();
                }
                Err(e) => info!("{}", e),
                _ => info!("some error")

            }
        }
        Ok(())
    }
}
