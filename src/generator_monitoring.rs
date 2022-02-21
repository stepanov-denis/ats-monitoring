pub mod generator {
    extern crate chrono;
    extern crate timer;
    use online::sync::check;
    use postgres::{Client, Error, NoTls};
    use std::sync::mpsc::channel;

    /// The structure of the generator failure.
    pub struct Faulty {
        generator_faulty: i32,
    }

    /// The structure of a UNIX timestamp with the time zone of the last value entry in the table.
    pub struct UnixFromSql {
        time: f64,
    }

    /// The structure of a UNIX timestamp with the time zone now.
    pub struct UnixFromSqlNow {
        time: f64,
    }

    /// The structure of the signal of the presence of the opc server connection with the plc.
    pub struct PlcConnect {
        connection: i32,
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
    pub fn generator_state() -> Result<(), Error> {
        let mut client =
            Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        // Checking the connection of the PostgreSQL DBMS with the OPC server.
        for row in client.query(
            "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let unix_from_sql = UnixFromSql { time: row.get(0) };
            for row in client
                .query("SELECT EXTRACT(epoch FROM now()) FROM avr_control_insert ORDER BY now() DESC limit 1", &[])
                ?
            {
                let unix_from_sql_now = UnixFromSqlNow { time: row.get(0) };
                let time_last_value = unix_from_sql.time + 5.00;
                if time_last_value > unix_from_sql_now.time {
                    // Сhecking the connection of the OPC server with the plc.
                    for row in client
                        .query("SELECT mark, connection FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                        ?
                    {
                        let plc_connect = PlcConnect {
                            connection: row.get(1),
                        };
                        if plc_connect.connection == 1 {
                            // Request for the health status of the generator.
                            for row in client
                                .query("SELECT mark, generator_faulty FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                ?
                            {
                                let faulty = Faulty {
                                    generator_faulty: row.get(1),
                                };
                                println!("Запрос работоспособности генератора в режиме трансляции питания от электросети");
                                println!("response from PostgreSQL: generator_faulty = {:?}", faulty.generator_faulty);
                                if faulty.generator_faulty == 1 {
                                    println!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                                    crate::psql::postgresql::event_generator_work_err();
                                    crate::psql::postgresql::log_generator_work_err();
                                    // Checking for internet access.
                                    if check(None).is_ok() {
                                        println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                        // Executing an http get request to the SMS gateway provider.
                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Авария!+Генератор+неисправен!+Срочно+произведите+сервисные+работы!").unwrap();
                                        if resp.status().is_success() {
                                            println!("Http запрос выполнен успешно");
                                            println!("Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913");
                                            crate::psql::postgresql::log_send_sms_generator_work_err();
                                            'inner: loop {
                                                // Checking the connection of the PostgreSQL DBMS with the OPC server.
                                                for row in client
                                                    .query(
                                                        "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
                                                        &[],
                                                    )
                                                    ?
                                                {
                                                    let unix_from_sql = UnixFromSql {
                                                        time: row.get(0),
                                                    };
                                                    for row in client
                                                        .query(
                                                            "SELECT EXTRACT(epoch FROM now()) FROM avr_control_insert ORDER BY now() DESC limit 1",
                                                            &[],
                                                        )
                                                        ?
                                                    {
                                                        let unix_from_sql_now = UnixFromSqlNow {
                                                            time: row.get(0),
                                                        };
                                                        let time_last_value = unix_from_sql.time + 5.00;
                                                        if time_last_value > unix_from_sql_now.time {
                                                            // Сhecking the connection of the OPC server with the plc.
                                                            for row in client
                                                                .query("SELECT mark, connection FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                                                ?
                                                            {
                                                                let plc_connect = PlcConnect {
                                                                    connection: row.get(1),
                                                                };
                                                                if plc_connect.connection == 1 {
                                                                    // Request for the health status of the generator.
                                                                    for row in client
                                                                        .query("SELECT mark, generator_faulty FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                                                        ?
                                                                    {
                                                                        let faulty = Faulty {
                                                                            generator_faulty: row.get(1),
                                                                        };
                                                                        println!("Запрос работоспособности генератора в режиме трансляции питаня от электросети");
                                                                        println!("response from PostgreSQL: generator_faulty = {:?}", faulty.generator_faulty);
                                                                        if faulty.generator_faulty == 0 {
                                                                            println!("Работоспособность генератора в режиме трансляции питания от электросети восстановлена");
                                                                            crate::psql::postgresql::event_generator_work_restored();
                                                                            crate::psql::postgresql::log_generator_work_restored();
                                                                            // Checking for internet access.
                                                                            if check(None).is_ok() {
                                                                                println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                                // Executing an http get request to the SMS gateway provider.
                                                                                let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Работоспособность+генератора+в+режиме+трансляции+питания+от+электросети+восстановлена.+Генератор+исправен.+Генератор+работает.").unwrap();
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
                                                                            break 'inner
                                                                        } else {
                                                                            println!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                                                                            crate::psql::postgresql::log_generator_work_err();
                                                                            timer_for_delay(3);
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                                                    crate::psql::postgresql::log_plc_err();
                                                                    timer_for_delay(3);
                                                                }
                                                            }
                                                        } else {
                                                            println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                                            crate::psql::postgresql::log_opc_err();
                                                            timer_for_delay(3);
                                                        }
                                                    }
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
                                    } else {
                                        println!("Ошибка! Доступ к интернету отсутствует!");
                                        println!("Ошибка! Http запрос не был выполнен!");
                                        println!("Ошибка! SMS уведомление не было отправлено!");
                                        crate::psql::postgresql::log_internet_err();
                                    }
                                } else {
                                    println!("Генератор в режиме трансляции питания от электросети работает исправно");
                                    crate::psql::postgresql::event_generator_work_ok();
                                    crate::psql::postgresql::log_generator_work_ok();
                                }
                            }
                        } else {
                            println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                            crate::psql::postgresql::log_plc_err();
                            timer_for_delay(3);
                        }
                    }
                } else {
                    println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                    crate::psql::postgresql::log_opc_err();
                    timer_for_delay(3);
                }
            }
        }
        Ok(())
    }
}
