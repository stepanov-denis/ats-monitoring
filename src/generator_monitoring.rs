pub mod generator {
    extern crate chrono;
    extern crate timer;
    use error_chain::error_chain;
    use online::sync::check;
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

    /// The function of determining the serviceability/malfunction of the generator and notifying about it by SMS using the gateway API.
    pub fn generator_state() -> Result<()> {
        if crate::skydb::skytable::unix_sql() + 5.00 > crate::skydb::skytable::unix_sql_now() {
            // Сhecking the connection of the OPC server with the plc.
            if crate::skydb::skytable::plc_connect() == 1 {
                // Request for the health status of the generator.
                info!("Запрос работоспособности генератора в режиме трансляции питания от электросети");
                info!(
                    "response from PostgreSQL: generator_faulty = {}",
                    crate::skydb::skytable::generator_faulty()
                );
                if crate::skydb::skytable::generator_faulty() == 1 {
                    info!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                    crate::psql::postgresql::event_generator_work_err();
                    crate::psql::postgresql::log_generator_work_err();
                    // Checking for internet access.
                    if check(None).is_ok() {
                        info!("Выполнение http запроса поставщику услуг SMS оповещения");
                        // Executing an http get request to the SMS gateway provider.
                        let resp =
                            reqwest::blocking::get(crate::sms::gateway::sms_generator_work_err())?;
                        if resp.status().is_success() {
                            info!("Http запрос выполнен успешно");
                            info!("Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                            crate::psql::postgresql::log_send_sms_generator_work_err();
                            'inner: loop {
                                // Checking the connection of the PostgreSQL DBMS with the OPC server
                                if crate::skydb::skytable::unix_sql() + 5.00
                                    > crate::skydb::skytable::unix_sql_now()
                                {
                                    // Сhecking the connection of the OPC server with the plc
                                    if crate::skydb::skytable::plc_connect() == 1 {
                                        // Request for the health status of the generator
                                        info!("Запрос работоспособности генератора в режиме трансляции питаня от электросети");
                                        info!(
                                            "response from PostgreSQL: generator_faulty = {}",
                                            crate::skydb::skytable::generator_faulty()
                                        );
                                        if crate::skydb::skytable::generator_faulty() == 0 {
                                            info!("Работоспособность генератора в режиме трансляции питания от электросети восстановлена");
                                            crate::psql::postgresql::event_generator_work_restored(
                                            );
                                            crate::psql::postgresql::log_generator_work_restored();
                                            // Checking for internet access.
                                            if check(None).is_ok() {
                                                info!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                // Executing an http get request to the SMS gateway provider.
                                                let resp = reqwest::blocking::get(
                                                    crate::sms::gateway::sms_generator_work_restored(),
                                                )?;
                                                if resp.status().is_success() {
                                                    info!("Http запрос выполнен успешно");
                                                    info!("Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена. Генератор исправен. Генератор работает./ на номер +79139402913");
                                                    crate::psql::postgresql::log_send_sms_generator_work_restored();
                                                } else if resp.status().is_server_error() {
                                                    info!("Server error!");
                                                    info!("Ошибка! SMS уведомление не было отправлено!");
                                                    crate::psql::postgresql::log_server_err();
                                                } else {
                                                    info!(
                                                        "Status http request: {}",
                                                        resp.status()
                                                    );
                                                    info!("Ошибка! SMS уведомление не было отправлено!");
                                                    crate::psql::postgresql::log_request_status_err(
                                                    );
                                                }
                                            } else {
                                                info!("Ошибка! Доступ к интернету отсутствует!");
                                                info!("Ошибка! Http запрос не был выполнен!");
                                                info!(
                                                    "Ошибка! SMS уведомление не было отправлено!"
                                                );
                                                crate::psql::postgresql::log_internet_err();
                                            }
                                            break 'inner;
                                        } else {
                                            info!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                                            crate::psql::postgresql::log_generator_work_err();
                                            crate::generator_monitoring::generator::timer_for_delay(
                                                3,
                                            );
                                        }
                                    } else {
                                        info!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                        crate::psql::postgresql::log_plc_err();
                                        crate::generator_monitoring::generator::timer_for_delay(3);
                                    }
                                } else {
                                    info!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                    crate::psql::postgresql::log_opc_err();
                                    crate::generator_monitoring::generator::timer_for_delay(3);
                                }
                            }
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
                        info!("Ошибка! Доступ к интернету отсутствует!");
                        info!("Ошибка! Http запрос не был выполнен!");
                        info!("Ошибка! SMS уведомление не было отправлено!");
                        crate::psql::postgresql::log_internet_err();
                    }
                } else {
                    info!(
                        "Генератор в режиме трансляции питания от электросети работает исправно"
                    );
                    crate::psql::postgresql::event_generator_work_ok();
                    crate::psql::postgresql::log_generator_work_ok();
                }
            } else {
                info!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                crate::psql::postgresql::log_plc_err();
                timer_for_delay(3);
            }
        } else {
            info!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
            crate::psql::postgresql::log_opc_err();
            timer_for_delay(3);
        }
        Ok(())
    }
}
