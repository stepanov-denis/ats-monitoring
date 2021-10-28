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

    /// Records the event "Авария! Генератор неисправен! Срочно произведите сервисные работы!" in the sql table "события_авр".
    pub fn event_generator_work_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Авария! Генератор неисправен! Срочно произведите сервисные работы!";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает." in the sql table "события_авр".
    pub fn event_generator_work_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event =
            "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Генератор в режиме трансляции питания от электросети работает исправно." in the sql table "события_авр".
    pub fn event_generator_work_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Генератор в режиме трансляции питания от электросети работает исправно.";
        client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records log "Авария! Генератор неисправен! Срочно произведите сервисные работы!" in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Авария! Генератор неисправен! Срочно произведите сервисные работы!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Генератор в режиме трансляции питания от электросети работает исправно." in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Генератор в режиме трансляции питания от электросети работает исправно.";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Работоспособность генератора в режиме трансляции питания от электросети восстановлена" in the sql table "журнал_работы_приложения".
    pub fn log_generator_work_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event =
            "Работоспособность генератора в режиме трансляции питания от электросети восстановлена";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_generator_work_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_generator_work_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event =
            "Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена./ на номер +79139402913";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Server error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_server_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Server error! Ошибка! SMS уведомление не было отправлено!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Http request status error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_request_status_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
        let event = "Http request status error! Ошибка! SMS уведомление не было отправлено!";
        client.execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
    pub fn log_internet_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls).unwrap();
        let event = "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            .unwrap();

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            .unwrap()
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь OPC сервера с ПЛК отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_plc_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls).unwrap();
        let event = "Ошибка! Связь Modbus клиента с ПЛК отсутствует!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            .unwrap();

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            .unwrap()
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Records log "Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_opc_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls).unwrap();
        let event = "Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            .unwrap();

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            .unwrap()
        {
            let event: &str = row.get(0);

            println!("Запись в табл. журнал_работы_приложения: {}", event);
        }
        Ok(())
    }

    /// Timer for delay 'inner: loop.
    fn timer_3sec() {
        let timer = timer::Timer::new();
        let (tx, rx) = channel();

        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
            tx.send(()).unwrap();
            let _ignored = tx.send(());
        });

        rx.recv().unwrap();
    }

    /// The function of determining the serviceability/malfunction of the generator and notifying about it by SMS using the gateway API.
    pub fn generator_state() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls)?;
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
                                    event_generator_work_err();
                                    log_generator_work_err();
                                    // Checking for internet access.
                                    if check(None).is_ok() {
                                        println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                        // Executing an http get request to the SMS gateway provider.
                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Авария!+Генератор+неисправен!+Срочно+произведите+сервисные+работы!").unwrap();
                                        if resp.status().is_success() {
                                            println!("Http запрос выполнен успешно");
                                            println!("Отправлено SMS сообщение: /Авария! Генератор неисправен! Срочно произведите сервисные работы!/ на номер +79139402913");
                                            log_send_sms_generator_work_err();
                                            'inner: loop {
                                                // Checking the connection of the PostgreSQL DBMS with the OPC server.
                                                for row in client
                                                    .query(
                                                        "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
                                                        &[],
                                                    )
                                                    .unwrap()
                                                {
                                                    let unix_from_sql = UnixFromSql {
                                                        time: row.get(0),
                                                    };
                                                    for row in client
                                                        .query(
                                                            "SELECT EXTRACT(epoch FROM now()) FROM avr_control_insert ORDER BY now() DESC limit 1",
                                                            &[],
                                                        )
                                                        .unwrap()
                                                    {
                                                        let unix_from_sql_now = UnixFromSqlNow {
                                                            time: row.get(0),
                                                        };
                                                        let time_last_value = unix_from_sql.time + 5.00;
                                                        if time_last_value > unix_from_sql_now.time {
                                                            // Сhecking the connection of the OPC server with the plc.
                                                            for row in client
                                                                .query("SELECT mark, connection FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                                                .unwrap()
                                                            {
                                                                let plc_connect = PlcConnect {
                                                                    connection: row.get(1),
                                                                };
                                                                if plc_connect.connection == 1 {
                                                                    // Request for the health status of the generator.
                                                                    for row in client
                                                                        .query("SELECT mark, generator_faulty FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                                                                        .unwrap()
                                                                    {
                                                                        let faulty = Faulty {
                                                                            generator_faulty: row.get(1),
                                                                        };
                                                                        println!("Запрос работоспособности генератора в режиме трансляции питаня от электросети");
                                                                        println!("response from PostgreSQL: generator_faulty = {:?}", faulty.generator_faulty);
                                                                        if faulty.generator_faulty == 0 {
                                                                            println!("Работоспособность генератора в режиме трансляции питания от электросети восстановлена");
                                                                            event_generator_work_restored();
                                                                            log_generator_work_restored();
                                                                            // Checking for internet access.
                                                                            if check(None).is_ok() {
                                                                                println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                                // Executing an http get request to the SMS gateway provider.
                                                                                let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Работоспособность+генератора+в+режиме+трансляции+питания+от+электросети+восстановлена.+Генератор+исправен.+Генератор+работает.").unwrap();
                                                                                if resp.status().is_success() {
                                                                                    println!("Http запрос выполнен успешно");
                                                                                    println!("Отправлено SMS сообщение: /Работоспособность генератора в режиме трансляции питания от электросети восстановлена. Генератор исправен. Генератор работает./ на номер +79139402913");
                                                                                    log_send_sms_generator_work_restored();
                                                                                } else if resp.status().is_server_error() {
                                                                                    println!("Server error!");
                                                                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                    log_server_err();
                                                                                } else {
                                                                                    println!("Status http request: {}", resp.status());
                                                                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                    log_request_status_err();
                                                                                }
                                                                            } else {
                                                                                println!("Ошибка! Доступ к интернету отсутствует!");
                                                                                println!("Ошибка! Http запрос не был выполнен!");
                                                                                println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                log_internet_err();
                                                                            }
                                                                            break 'inner
                                                                        } else {
                                                                            println!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                                                                            log_generator_work_err();
                                                                            timer_3sec();
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                                                                    log_plc_err();
                                                                }
                                                            }
                                                        } else {
                                                            println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                                                            log_opc_err();
                                                        }
                                                    }
                                                }
                                            }
                                        } else if resp.status().is_server_error() {
                                            println!("Server error!");
                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                            log_server_err();
                                        } else {
                                            println!("Status http request: {}", resp.status());
                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                            log_request_status_err();
                                        }
                                    } else {
                                        println!("Ошибка! Доступ к интернету отсутствует!");
                                        println!("Ошибка! Http запрос не был выполнен!");
                                        println!("Ошибка! SMS уведомление не было отправлено!");
                                        log_internet_err();
                                    }
                                } else {
                                    println!("Генератор в режиме трансляции питания от электросети работает исправно");
                                    event_generator_work_ok();
                                    log_generator_work_ok();
                                }
                            }
                        } else {
                            println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                            log_plc_err();
                        }
                    }
                } else {
                    println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
                    log_opc_err();
                }
            }
        }
        Ok(())
    }
}
