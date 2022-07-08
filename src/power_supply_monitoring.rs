pub mod power_supply {
    extern crate chrono;
    extern crate timer;
    // use error_chain::error_chain;
    use online::sync::check;
    use std::error;
    use std::sync::mpsc::channel;

    // error_chain! {
    //     foreign_links {
    //         Io(std::io::Error);
    //         HttpRequest(reqwest::Error);
    //     }
    // }

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

    /// Main spawn - the function for detecting a power failure from the mains/restoring power from the mains,
    /// successful start of the generator, failure of the generator start, and notifications about these events.
    /// Additional spawn - the function of determining the serviceability/malfunction of the generator
    /// and notifying about it by SMS using the gateway API.
    pub fn ats_state() -> Result<(), Box<dyn error::Error + Send + Sync>> {
        if crate::skydb::skytable::unix_sql() + 5.00 >= crate::skydb::skytable::unix_sql_now() {
            // Сhecking the connection of the OPC server with the plc.
            if crate::skydb::skytable::plc_connect() == 1 {
                // Request for the presence of a power failure from the power grid.
                info!("Запрос наличия питания от электросети");
                info!(
                    "response from PostgreSQL: mains_power_supply = {}",
                    crate::skydb::skytable::mains_power_supply()
                );
                if crate::skydb::skytable::mains_power_supply() == 0 {
                    info!("Произошел сбой питания от электросети");
                    info!("Ожидание (90 секунд) подтверждения отсутствия питания от электросети");
                    crate::psql::postgresql::log_power_failure();
                    timer_for_delay(90);
                    // Checking the connection of the PostgreSQL DBMS with the OPC server.
                    if crate::skydb::skytable::unix_sql() + 5.00
                        >= crate::skydb::skytable::unix_sql_now()
                    {
                        // Сhecking the connection of the OPC server with the plc.
                        if crate::skydb::skytable::plc_connect() == 1 {
                            // Request for the availability of power from the mains and request the start status of the generator.
                            info!("Повторный запрос наличия питания от электросети");
                            info!(
                                "response from PostgreSQL: mains_power_supply = {}",
                                crate::skydb::skytable::mains_power_supply()
                            );
                            if crate::skydb::skytable::mains_power_supply() == 0 {
                                info!("Подтверждение отсутствия питания от электросети");
                                crate::psql::postgresql::log_power_failure_confirmed();
                                // Checking internet access.
                                if check(None).is_ok() {
                                    info!(
                                        "Выполнение http запроса поставщику услуг SMS оповещения"
                                    );
                                    if crate::skydb::skytable::start_generator() == 1 {
                                        info!("Успешный старт генератора");
                                        crate::psql::postgresql::event_power_failure_start_generator_ok();
                                        crate::psql::postgresql::log_start_generator_ok();
                                        // Executing an http get request to the SMS gateway provider.
                                        let resp = reqwest::blocking::get(
                                            crate::alerts::sms_gateway::sms_message(
                                                "SMS_START_GEN_OK",
                                            )
                                            .unwrap_or_default(),
                                        )?;
                                        if resp.status().is_success() {
                                            info!("Http запрос выполнен успешно");
                                            info!("Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913");
                                            crate::psql::postgresql::log_send_sms_start_generator_ok();
                                        } else if resp.status().is_server_error() {
                                            info!("Server error!");
                                            info!("Ошибка! SMS уведомление не было отправлено!");
                                            crate::psql::postgresql::log_server_err();
                                        } else {
                                            info!("Status http request: {}", resp.status());
                                            info!("Ошибка! SMS уведомление не было отправлено!");
                                            crate::psql::postgresql::log_request_status_err();
                                        }
                                    } else {
                                        info!("Сбой старта генератора!");
                                        crate::psql::postgresql::event_power_failure_start_generator_err();
                                        crate::psql::postgresql::log_start_generator_err();
                                        // Executing an http get request to the SMS gateway provider.
                                        let resp = reqwest::blocking::get(
                                            crate::alerts::sms_gateway::sms_message(
                                                "SMS_START_GEN_ERR",
                                            )
                                            .unwrap_or_default(),
                                        )?;
                                        if resp.status().is_success() {
                                            info!("Http запрос выполнен успешно");
                                            info!("Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913");
                                            crate::psql::postgresql::log_send_sms_start_generator_err();
                                        } else if resp.status().is_server_error() {
                                            info!("Server error!");
                                            info!("Ошибка! SMS уведомление не было отправлено!");
                                            crate::psql::postgresql::log_server_err();
                                        } else {
                                            info!("Status http request: {}", resp.status());
                                            info!("Ошибка! SMS уведомление не было отправлено!");
                                            crate::psql::postgresql::log_request_status_err();
                                        }
                                    }
                                    'inner: loop {
                                        // Checking the connection of the PostgreSQL DBMS with the OPC server.
                                        if crate::skydb::skytable::unix_sql() + 5.00
                                            >= crate::skydb::skytable::unix_sql_now()
                                        {
                                            // Сhecking the connection of the OPC server with the plc.
                                            if crate::skydb::skytable::plc_connect() == 1 {
                                                // Request for the availability of power from the mains and request the status of the generator.
                                                info!("Запрос наличия питания от электросети");
                                                info!("response from PostgreSQL: mains_power_supply = {}", crate::skydb::skytable::mains_power_supply());
                                                if crate::skydb::skytable::mains_power_supply() == 1
                                                {
                                                    info!("Питание от электросети восстановлено");
                                                    crate::psql::postgresql::log_power_restored();
                                                    // Checking internet access.
                                                    if check(None).is_ok() {
                                                        info!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                        if crate::skydb::skytable::generator_work()
                                                            == 1
                                                        {
                                                            info!("Генератор исправен. Генератор работает");
                                                            crate::psql::postgresql::event_power_restored_generator_work_ok();
                                                            crate::psql::postgresql::log_power_restored_generator_ok();
                                                            // Executing an http get request to the SMS gateway provider.
                                                            let resp = reqwest::blocking::get(crate::alerts::sms_gateway::sms_message("SMS_POW_RESTORED_GEN_OK").unwrap_or_default())?;
                                                            if resp.status().is_success() {
                                                                info!(
                                                                    "Http запрос выполнен успешно"
                                                                );
                                                                info!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913");
                                                                crate::psql::postgresql::log_send_sms_power_restored_generator_ok();
                                                            } else if resp
                                                                .status()
                                                                .is_server_error()
                                                            {
                                                                info!("Server error!");
                                                                info!("Ошибка! SMS уведомление не было отправлено!");
                                                                crate::psql::postgresql::log_server_err();
                                                            } else {
                                                                info!(
                                                                    "Status http request: {}",
                                                                    resp.status()
                                                                );
                                                                info!("Ошибка! SMS уведомление не было отправлено!");
                                                                crate::psql::postgresql::log_request_status_err();
                                                            }
                                                        } else {
                                                            info!("Генератор неисправен. Генератор не работает");
                                                            crate::psql::postgresql::event_power_restored_generator_work_err();
                                                            crate::psql::postgresql::log_power_restored_generator_err();
                                                            // Executing an http get request to the SMS gateway provider.
                                                            let resp = reqwest::blocking::get(crate::alerts::sms_gateway::sms_message("SMS_POW_RESTORED_GEN_ERR").unwrap_or_default())?;
                                                            if resp.status().is_success() {
                                                                info!(
                                                                    "Http запрос выполнен успешно"
                                                                );
                                                                info!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913");
                                                                crate::psql::postgresql::log_send_sms_power_restored_generator_err();
                                                            } else if resp
                                                                .status()
                                                                .is_server_error()
                                                            {
                                                                info!("Server error!");
                                                                info!("Ошибка! SMS уведомление не было отправлено!");
                                                                crate::psql::postgresql::log_server_err();
                                                            } else {
                                                                info!(
                                                                    "Status http request: {}",
                                                                    resp.status()
                                                                );
                                                                info!("Ошибка! SMS уведомление не было отправлено!");
                                                                crate::psql::postgresql::log_request_status_err();
                                                            }
                                                        }
                                                    } else {
                                                        info!("Ошибка! Доступ к интернету отсутствует!");
                                                        info!(
                                                            "Ошибка! Http запрос не был выполнен!"
                                                        );
                                                        info!("Ошибка! SMS уведомление не было отправлено!");
                                                        crate::psql::postgresql::log_internet_err();
                                                    }
                                                    break 'inner;
                                                } else {
                                                    info!("Питание от электросети еще не было восстановлено, после отключения");
                                                    crate::psql::postgresql::log_power_dont_restored();
                                                }
                                            } else {
                                                info!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                                crate::psql::postgresql::log_plc_err();
                                            }
                                        } else {
                                            info!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                            crate::psql::postgresql::log_opc_err();
                                        }
                                    }
                                } else {
                                    info!("Ошибка! Доступ к сети интернет осутствует!");
                                    info!("Ошибка! Http запрос не был выполнен!");
                                    info!("Ошибка! SMS уведомление не было отправлено!");
                                    crate::psql::postgresql::log_internet_err();
                                }
                            } else {
                                info!("Питание от электросети восстановлено");
                                crate::psql::postgresql::log_power_restored();
                            }
                        } else {
                            info!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                            crate::psql::postgresql::log_plc_err();
                        }
                    } else {
                        info!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                        crate::psql::postgresql::log_opc_err();
                    }
                } else {
                    info!("Питание от электросети есть");
                    crate::psql::postgresql::event_power_ok();
                    crate::psql::postgresql::log_power_ok();
                }
            } else {
                info!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                crate::psql::postgresql::log_plc_err();
            }
        } else {
            info!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
            crate::psql::postgresql::log_opc_err();
        }
        Ok(())
    }
}
