pub mod http {
    use error_chain::error_chain;
    use online::sync::check;
    
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    pub fn generator_monitoring_cycle() -> Result<()> {
        let resp = reqwest::blocking::get(crate::sms::gateway::sms_generator_work_err())?;
        if resp.status().is_success() {
            println!("Http запрос выполнен успешно");
            println!("Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!");
            crate::psql::postgresql::log_send_sms_generator_work_err();
            'inner: loop {
                // Checking the connection of the PostgreSQL DBMS with the OPC server
                if crate::skydb::skytable::unix_sql() + 5.00
                    > crate::skydb::skytable::unix_sql_now()
                {
                    // Сhecking the connection of the OPC server with the plc
                    if crate::skydb::skytable::plc_connect() == 1 {
                        // Request for the health status of the generator
                        println!("Запрос работоспособности генератора в режиме трансляции питаня от электросети");
                        println!(
                            "response from PostgreSQL: generator_faulty = {}",
                            crate::skydb::skytable::generator_faulty()
                        );
                        if crate::skydb::skytable::generator_faulty() == 0 {
                            println!("Работоспособность генератора в режиме трансляции питания от электросети восстановлена");
                            crate::psql::postgresql::event_generator_work_restored();
                            crate::psql::postgresql::log_generator_work_restored();
                            // Checking for internet access.
                            if check(None).is_ok() {
                                println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                // Executing an http get request to the SMS gateway provider.
                                let resp = reqwest::blocking::get(
                                    crate::sms::gateway::sms_generator_work_restored(),
                                )?;
                                if resp.status().is_success() {
                                    println!("Http запрос выполнен успешно");
                                    println!("Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена. Генератор исправен. Генератор работает./ на номер +79139402913");
                                    crate::psql::postgresql::log_send_sms_generator_work_restored();
                                } else if resp.status().is_server_error() {
                                    println!("Server error!");
                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                    crate::psql::postgresql::log_server_err();
                                } else {
                                    println!("Status http request: {}", resp.status());
                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                    crate::psql::postgresql::log_request_status_err();
                                }
                            } else {
                                println!("Ошибка! Доступ к интернету отсутствует!");
                                println!("Ошибка! Http запрос не был выполнен!");
                                println!("Ошибка! SMS уведомление не было отправлено!");
                                crate::psql::postgresql::log_internet_err();
                            }
                            break 'inner;
                        } else {
                            println!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                            crate::psql::postgresql::log_generator_work_err();
                            crate::generator_monitoring::generator::timer_for_delay(3);
                        }
                    } else {
                        println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                        crate::psql::postgresql::log_plc_err();
                        crate::generator_monitoring::generator::timer_for_delay(3);
                    }
                } else {
                    println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                    crate::psql::postgresql::log_opc_err();
                    crate::generator_monitoring::generator::timer_for_delay(3);
                }
            }
        } else if resp.status().is_server_error() {
            println!("Server error!");
            println!("Ошибка! SMS уведомление не было отправлено!");
            crate::psql::postgresql::log_server_err();
        } else {
            println!("Status http request: {}", resp.status());
            println!("Ошибка! SMS уведомление не было отправлено!");
            crate::psql::postgresql::log_request_status_err();
        }
        Ok(())
    }
}
