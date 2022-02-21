pub mod power_supply {
    extern crate chrono;
    extern crate timer;
    use online::sync::check;
    use postgres::{Client, Error, NoTls};

    /// The structure of power supply from the power grid.
    pub struct PowerSupply {
        mains_power_supply: i32,
        start_generator: i32,
        generator_work: i32,
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

    /// Main spawn - the function for detecting a power failure from the mains/restoring power from the mains,
    /// successful start of the generator, failure of the generator start, and notifications about these events.
    /// Additional spawn - the function of determining the serviceability/malfunction of the generator
    /// and notifying about it by SMS using the gateway API.
    pub fn ats_state() -> Result<(), Error> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        // Checking the connection of the PostgreSQL DBMS with the OPC server.
        for row in client.query(
            "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let unix_from_sql = UnixFromSql { time: row.get(0) };
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
                            // Request for the presence of a power failure from the power grid.
                            for row in client
                                .query("SELECT mains_power_supply, start_generator, generator_work, mark FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                ?
                            {
                                let powersupply = PowerSupply {
                                    mains_power_supply: row.get(0),
                                    start_generator: row.get(1),
                                    generator_work: row.get(2),
                                };

                                println!("Запрос наличия питания от электросети");
                                println!("response from PostgreSQL: mains_power_supply = {:?}", powersupply.mains_power_supply);

                                if powersupply.mains_power_supply == 0 {
                                    println!("Произошел сбой питания от электросети");
                                    println!("Ожидание (90 секунд) подтверждения отсутствия питания от электросети");
                                    crate::psql::postgresql::log_power_failure();
                                    crate::generator_monitoring::generator::timer_for_delay(90);
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
                                                        // Request for the availability of power from the mains and request the start status of the generator.
                                                        for row in client
                                                            .query("SELECT mains_power_supply, start_generator, generator_work, mark FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                                            ?
                                                        {
                                                            let powersupply = PowerSupply {
                                                                mains_power_supply: row.get(0),
                                                                start_generator: row.get(1),
                                                                generator_work: row.get(2),
                                                            };
                                                            println!("Повторный запрос наличия питания от электросети");
                                                            println!("response from PostgreSQL: mains_power_supply = {:?}", powersupply.mains_power_supply);
                                                            if powersupply.mains_power_supply == 0 {
                                                                println!("Подтверждение отсутствия питания от электросети");
                                                                crate::psql::postgresql::log_power_failure_confirmed();
                                                                // Checking internet access.
                                                                if check(None).is_ok() {
                                                                    println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                    if powersupply.start_generator == 1 {
                                                                        println!("Успешный старт генератора");
                                                                        crate::psql::postgresql::event_power_failure_start_generator_ok();
                                                                        crate::psql::postgresql::log_start_generator_ok();
                                                                        // Executing an http get request to the SMS gateway provider.
                                                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Успешный+старт+генератора.").unwrap();
                                                                        if resp.status().is_success() {
                                                                            println!("Http запрос выполнен успешно");
                                                                            println!("Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913");
                                                                            crate::psql::postgresql::log_send_sms_start_generator_ok();
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
                                                                        println!("Сбой старта генератора!");
                                                                        crate::psql::postgresql::event_power_failure_start_generator_err();
                                                                        crate::psql::postgresql::log_start_generator_err();
                                                                        // Executing an http get request to the SMS gateway provider.
                                                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Сбой+старта+генератора.").unwrap();
                                                                        if resp.status().is_success() {
                                                                            println!("Http запрос выполнен успешно");
                                                                            println!("Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913");
                                                                            crate::psql::postgresql::log_send_sms_start_generator_err();
                                                                        } else if resp.status().is_server_error() {
                                                                            println!("Server error!");
                                                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                                                            crate::psql::postgresql::log_server_err();
                                                                        } else {
                                                                            println!("Status http request: {}", resp.status());
                                                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                                                            crate::psql::postgresql::log_request_status_err();
                                                                        }
                                                                    }
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
                                                                                            // Request for the availability of power from the mains and request the status of the generator.
                                                                                            for row in client
                                                                                                .query(
                                                                                                    "SELECT mains_power_supply, start_generator, generator_work, mark FROM avr_control_insert ORDER BY mark DESC limit 1",
                                                                                                    &[],
                                                                                                )
                                                                                                ?
                                                                                            {
                                                                                                let powersupply = PowerSupply {
                                                                                                    mains_power_supply: row.get(0),
                                                                                                    start_generator: row.get(1),
                                                                                                    generator_work: row.get(2),
                                                                                                };
                                                                                                println!("Запрос наличия питания от электросети");
                                                                                                println!("response from PostgreSQL: mains_power_supply = {:?}", powersupply.mains_power_supply);
                                                                                                if powersupply.mains_power_supply == 1 {
                                                                                                    println!("Питание от электросети восстановлено");
                                                                                                    crate::psql::postgresql::log_power_restored();
                                                                                                    // Checking internet access.
                                                                                                    if check(None).is_ok() {
                                                                                                        println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                                                        if powersupply.generator_work == 1 {
                                                                                                            println!("Генератор исправен. Генератор работает");
                                                                                                            crate::psql::postgresql::event_power_restored_generator_work_ok();
                                                                                                            crate::psql::postgresql::log_power_restored_generator_ok();
                                                                                                            // Executing an http get request to the SMS gateway provider.
                                                                                                            let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Питание+от+электросети+восстановлено.+Генератор+исправен.+Генератор+работает.").unwrap();
                                                                                                            if resp.status().is_success() {
                                                                                                                println!("Http запрос выполнен успешно");
                                                                                                                println!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913");
                                                                                                                crate::psql::postgresql::log_send_sms_power_restored_generator_ok();
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
                                                                                                            println!("Генератор неисправен. Генератор не работает");
                                                                                                            crate::psql::postgresql::event_power_restored_generator_work_err();
                                                                                                            crate::psql::postgresql::log_power_restored_generator_err();
                                                                                                            // Executing an http get request to the SMS gateway provider.
                                                                                                            let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Питание+от+электросети+восстановлено.+Генератор+неисправен.+Генератор+не+работает.").unwrap();
                                                                                                            if resp.status().is_success() {
                                                                                                                println!("Http запрос выполнен успешно");
                                                                                                                println!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913");
                                                                                                                crate::psql::postgresql::log_send_sms_power_restored_generator_err();
                                                                                                            } else if resp.status().is_server_error() {
                                                                                                                println!("Server error!");
                                                                                                                println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                                crate::psql::postgresql::log_server_err();
                                                                                                            } else {
                                                                                                                println!("Status http request: {}", resp.status());
                                                                                                                println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                                crate::psql::postgresql::log_request_status_err();
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        println!("Ошибка! Доступ к интернету отсутствует!");
                                                                                                        println!("Ошибка! Http запрос не был выполнен!");
                                                                                                        println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                        crate::psql::postgresql::log_internet_err();
                                                                                                    }
                                                                                                    break 'inner;
                                                                                                } else {
                                                                                                    println!("Питание от электросети еще не было восстановлено, после отключения");
                                                                                                    crate::psql::postgresql::log_power_dont_restored();
                                                                                                    crate::generator_monitoring::generator::timer_for_delay(3);
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                                                                            crate::psql::postgresql::log_plc_err();
                                                                                            crate::generator_monitoring::generator::timer_for_delay(3);
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                                                                    crate::psql::postgresql::log_opc_err();
                                                                                    crate::generator_monitoring::generator::timer_for_delay(3);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("Ошибка! Доступ к сети интернет осутствует!");
                                                                    println!("Ошибка! Http запрос не был выполнен!");
                                                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                                                    crate::psql::postgresql::log_internet_err();
                                                                }
                                                            } else {
                                                                println!("Питание от электросети восстановлено");
                                                                crate::psql::postgresql::log_power_restored();
                                                            }
                                                        }
                                                    } else {
                                                        println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                                        crate::psql::postgresql::log_plc_err();
                                                    }
                                                }
                                            } else {
                                                println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                                crate::psql::postgresql::log_opc_err();
                                            }
                                        }
                                    }
                                } else {
                                    println!("Питание от электросети есть");
                                    crate::psql::postgresql::event_power_ok();
                                    crate::psql::postgresql::log_power_ok();
                                }
                            }
                        } else {
                            println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                            crate::psql::postgresql::log_plc_err();
                            crate::generator_monitoring::generator::timer_for_delay(3);
                        }
                    }
                } else {
                    println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                    crate::psql::postgresql::log_opc_err();
                    crate::generator_monitoring::generator::timer_for_delay(3);
                }
            }
        }
        Ok(())
    }
}
