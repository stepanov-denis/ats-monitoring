pub mod generator {
    extern crate chrono;
    extern crate timer;
    use error_chain::error_chain;
    use std::sync::mpsc::channel;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Timer for delay 'inner: loop.
    pub fn timer_for_delay(sec: i64) {
        let timer = timer::Timer::new();
        let (tx, rx) = channel();

        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(sec), move || {
            tx.send(()).unwrap();
            let _ignored = tx.send(());
        });

        rx.recv().unwrap();
    }

    /// Check connection app to plc and postgresql
    pub fn check_connect_app_to_db_and_plc() -> Option<bool> {
        match crate::skydb::skytable::unix_sql() + 5.00 >= crate::skydb::skytable::unix_sql_now() {
            false => {
                info!("connection app to postgresql: error");
                match crate::psql::postgresql::log_opc_err() {
                    Ok(_) => info!("connection app to postgresql: ok"),
                    Err(e) => info!("{}", e),
                }

                Some(false)
            }
            true => check_connect_app_to_plc(),
        }
    }

    pub fn check_connect_app_to_plc() -> Option<bool> {
        match crate::skydb::skytable::plc_connect() == 1 {
            false => {
                info!("connection app to plc: error");
                match crate::psql::postgresql::log_plc_err() {
                    Ok(_) => info!("log_plc_err(): ok"),
                    Err(e) => info!("{}", e),
                }
                Some(false)
            }
            true => {
                info!("connection app to plc and postgresql: ok");
                Some(true)
            }
        }
    }

    /// Logging event "Alarm! The generator is faulty! Urgently perform service work!"
    pub fn log_alarm() {
        info!("Alarm! The generator is faulty! Urgently perform service work!");
        if crate::psql::postgresql::event_generator_work_err().is_ok() {
            info!("event_generator_work_err(): ok");
        } else {
            info!("event_generator_work_err(): error");
        }
        if crate::psql::postgresql::log_generator_work_err().is_ok() {
            info!("log_generator_work_err(): ok");
        } else {
            info!("log_generator_work_err(): error");
        }
    }

    /// Logging event "server error the sms notification was not sent"
    pub fn log_sms_gateway_server_error(response: reqwest::blocking::Response) {
        info!("status http request: {}", response.status());
        info!("server error the sms notification was not sent");
        if crate::psql::postgresql::log_server_err().is_ok() {
            info!("log_server_err(): ok");
        } else {
            info!("log_server_err(): error");
        }
    }

    /// Logging request for operation of the generator
    /// in the mode of transmission of electricity from the power grid
    pub fn log_request_to_generator() {
        info!(
            "request for operation of the generator 
            in the mode of transmission of electricity from the power grid"
        );

        info!(
            "response from postgresql: generator_faulty = {}",
            generator_faulty = crate::skydb::skytable::generator_faulty()
        );
    }

    /// Inner loop for cyclic polling of the emergency generator
    pub fn inner_loop_generator_faulty() -> Result<()> {
        'inner: loop {
            if check_connect_app_to_db_and_plc() == Some(true) {
                log_request_to_generator();
                if crate::skydb::skytable::generator_faulty() == 0 {
                    info!(
                        "the efficiency of the generator in the mode 
                        of transmission of electricity from the power grid has been restored"
                    );
                    if crate::psql::postgresql::event_generator_work_restored().is_ok() {
                        info!("event_generator_work_restored(): ok");
                    } else {
                        info!("event_generator_work_restored(): error");
                    }
                    if crate::psql::postgresql::log_generator_work_restored().is_ok() {
                        info!("log_generator_work_restored(): ok");
                    } else {
                        info!("log_generator_work_restored(): error");
                    }
                    info!("executing an http request to an SMS notification service provider");
                    let resp = reqwest::blocking::get(
                        crate::alerts::gateway::sms_generator_work_restored(),
                    )?;
                    if resp.status().is_success() {
                        info!("http request completed successfully");
                        info!("an sms message was sent: Работоспособность генератора в режиме трансляции питания от электросети восстановлена. Генератор исправен. Генератор работает.");
                        if crate::psql::postgresql::log_send_sms_generator_work_restored().is_ok() {
                            info!("log_send_sms_generator_work_restored(): ok");
                        } else {
                            info!("log_send_sms_generator_work_restored(): error");
                        }
                    } else {
                        log_sms_gateway_server_error(resp);
                    }
                    break 'inner;
                } else {
                    log_alarm();
                }
            }
        }
        Ok(())
    }

    /// The function of determining the serviceability/malfunction
    /// of the generator and notifying about it by SMS using the gateway API.
    pub fn generator_state() -> Result<()> {
        if check_connect_app_to_db_and_plc() == Some(true) {
            log_request_to_generator();
            if crate::skydb::skytable::generator_faulty() == 1 {
                log_alarm();
                info!("executing an http request to an SMS notification service provider");
                let resp =
                    reqwest::blocking::get(crate::alerts::gateway::sms_generator_work_err())?;
                if resp.status().is_success() {
                    info!("http request completed successfully");
                    info!(
                        "an sms message was sent: 
                    Авария! Генератор неисправен! Срочно произведите сервисные работы!"
                    );
                    if crate::psql::postgresql::log_send_sms_generator_work_err().is_ok() {
                        info!("log_send_sms_generator_work_err(): ok");
                    } else {
                        info!("log_send_sms_generator_work_err(): error");
                    }
                    if inner_loop_generator_faulty().is_ok() {
                        info!("inner_loop(): ok");
                    } else {
                        info!("inner_loop(): error");
                    }
                } else {
                    log_sms_gateway_server_error(resp);
                }
            } else {
                info!("generator is working properly in the mode of electricity transmission from the power grid");
                if crate::psql::postgresql::event_generator_work_ok().is_ok() {
                    info!("event_generator_work_ok(): ok");
                } else {
                    info!("event_generator_work_ok(): error");
                }
                if crate::psql::postgresql::log_generator_work_ok().is_ok() {
                    info!("log_generator_work_ok(): ok");
                } else {
                    info!("log_generator_work_ok(): error");
                }
            }
        }
        Ok(())
    }
}
