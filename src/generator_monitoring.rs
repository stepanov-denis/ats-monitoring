pub mod generator {
    extern crate chrono;
    extern crate timer;
    use error_chain::error_chain;
    use reqwest::StatusCode;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    /// Logging event "Alarm! The generator is faulty! Urgently perform service work!".
    pub fn log_alarm() {
        info!("Alarm! The generator is faulty! Urgently perform service work!");
        // Records the event
        // "Авария! Генератор неисправен! Срочно произведите сервисные работы!"
        // in the sql table "события_авр".
        match crate::psql::postgresql::event_generator_work_err() {
            Ok(_) => info!("crate::psql::postgresql::event_generator_work_err(): ok"),
            Err(e) => info!("{}", e)
        }
        // Records log
        // "Авария! Генератор неисправен! Срочно произведите сервисные работы!"
        // in the sql table "журнал_работы_приложения".
        match crate::psql::postgresql::log_generator_work_err() {
            Ok(_) => info!("crate::psql::postgresql::log_generator_work_err(): ok"),
            Err(e) => info!("{}", e)
        }
    }

    /// Logging event "server error the sms notification was not sent".
    pub fn log_sms_gateway_server_error(response: reqwest::blocking::Response) {
        info!("status http request: {}", response.status());
        info!("server error the sms notification was not sent");
        // Records log
        // "Server error! Ошибка! SMS уведомление не было отправлено!"
        // in the sql table "журнал_работы_приложения".
        match crate::psql::postgresql::log_server_err() {
            Ok(_) => info!("crate::psql::postgresql::log_server_err(): ok"),
            Err(e) => info!("{}", e)
        }
    }

    /// Logging request for operation of the generator
    /// in the mode of transmission of electricity from the power grid.
    pub fn log_request_to_generator() {
        info!(
            "request for operation of the generator 
            in the mode of transmission of electricity from the power grid"
        );
        info!(
            "response from postgresql: generator_faulty = {:?}",
            crate::psql::postgresql::select_generator_faulty()
        );
    }

    /// Sending SMS.
    pub fn send_sms(message_env: &str) -> Result<()> {
        info!("executing an http request to an sms notification service provider");
        let resp = reqwest::blocking::get(
            crate::alerts::sms_gateway::sms_message(message_env).unwrap_or_default(),
        )?;
        match resp.status() {
            StatusCode::OK => {
                info!("http request completed successfully");
                info!(
                    "an sms message was sent: {:?}",
                    crate::read_env::env::read_str(message_env)
                );
                // Records log 
                // "Отправлено SMS сообщение:
                // Работоспособность генератора в режиме трансляции питания 
                // от электросети восстановлена.
                // in the sql table "журнал_работы_приложения".
                match crate::psql::postgresql::log_send_sms_generator_work_restored(message_env) {
                    Ok(_) => info!("crate::psql::postgresql::log_send_sms_generator_work_restored(message_env): ok"),
                    Err(e) => info!("{}", e)
                }
            }
            _ => log_sms_gateway_server_error(resp)
        }
        Ok(())
    }

    /// Inner loop for cyclic polling of the emergency generator.
    pub fn inner_loop_generator_faulty() {
        'inner: loop {
            // Checking the connection of the app to the PLC.
            if crate::modbus_ats::avr_control::reading_connection() == Some(true) {
                log_request_to_generator();
                // Checking the faulty condition of the generator
                // 0 => the generator is working properly
                // 1 => the generator is faulty.
                match crate::psql::postgresql::select_generator_faulty() {
                    Ok(0) => {
                        info!(
                            "the efficiency of the generator in the mode 
                            of transmission of electricity from the power grid has been restored"
                        );
                        // Records the event
                        // "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает."
                        // in the sql table "события_авр".
                        match crate::psql::postgresql::event_generator_work_restored() {
                            Ok(_) => info!("crate::psql::postgresql::event_generator_work_restored(): ok"),
                            Err(e) => info!("{}", e)
                        }
                        // Records log 
                        // "Работоспособность генератора в режиме трансляции питания от электросети восстановлена"
                        // in the sql table "журнал_работы_приложения".
                        match crate::psql::postgresql::log_generator_work_restored() {
                            Ok(_) => info!("crate::psql::postgresql::log_generator_work_restored(): ok"),
                            Err(e) => info!("{}", e)
                        }
                        // Sending SMS notification.
                        match send_sms("SMS_GEN_WORK_RESTORED") {
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
    pub fn generator_state() -> Result<()> {
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
                    match send_sms("SMS_GEN_WORK_ERR") {
                        Ok(_) => info!("send_sms('SMS_GEN_WORK_ERR'): ok"),
                        Err(e) => info!("{}", e),
                    }
                    // Entering the generator polling cycle when a fault is detected.
                    inner_loop_generator_faulty();
                }
                _ => {
                    info!("generator is working properly in the mode of electricity transmission from the power grid");
                    // Records the event
                    // "Генератор в режиме трансляции питания от электросети работает исправно."
                    // in the sql table "события_авр".
                    match crate::psql::postgresql::event_generator_work_ok() {
                        Ok(_) => info!("crate::psql::postgresql::event_generator_work_ok(): ok"),
                        Err(e) => info!("{}", e)
                    }
                     // Records log
                     // "Генератор в режиме трансляции питания от электросети работает исправно."
                     // in the sql table "журнал_работы_приложения".
                    match crate::psql::postgresql::log_generator_work_ok() {
                        Ok(_) => info!("crate::psql::postgresql::log_generator_work_ok(): ok"),
                        Err(e) => info!("{}", e)
                    }
                }
            }
        }
        Ok(())
    }
}
