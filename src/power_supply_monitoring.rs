pub mod power_supply {
    extern crate chrono;
    extern crate timer;
    use online::sync::check;
    use postgres::{Client, NoTls, Error};
    use std::sync::mpsc::channel;

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

    /// Records the event "Питание от электросети есть." in the sql table "события_авр".
    pub fn event_power_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Питание от электросети есть.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Сбой питания от электросети. Успешный старт генератора." in the sql table "события_авр".
    pub fn event_power_failure_start_generator_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Сбой питания от электросети. Успешный старт генератора.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Сбой питания от электросети. Сбой старта генератора." in the sql table "события_авр".
    pub fn event_power_failure_start_generator_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).unwrap();
        let event = "Сбой питания от электросети. Сбой старта генератора.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Питание от электросети восстановлено. Генератор исправен. Генератор работает." in the sql table "события_авр".
    pub fn event_power_restored_generator_work_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Питание от электросети восстановлено. Генератор исправен. Генератор работает.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Питание от электросети восстановлено. Генератор неисправен. Генератор не работает." in the sql table "события_авр".
    pub fn event_power_restored_generator_work_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event =
            "Питание от электросети восстановлено. Генератор неисправен. Генератор не работает.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Авария! Генератор неисправен! Срочно произведите сервисные работы!" in the sql table "события_авр".
    pub fn event_generator_work_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Авария! Генератор неисправен! Срочно произведите сервисные работы!";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает." in the sql table "события_авр".
    pub fn event_generator_work_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event =
            "Работоспособность генератора восстановлена. Генератор исправен. Генератор работает.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records the event "Генератор в режиме трансляции питания от электросети работает исправно." in the sql table "события_авр".
    pub fn event_generator_work_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Генератор в режиме трансляции питания от электросети работает исправно.";
        client
            .execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])
            ?;

        for row in client
            .query(
                "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            ?
        {
            let event: &str = row.get(0);

            println!("Запись в табл. события_авр: {}", event);
        }
        Ok(())
    }

    /// Records log "Произошел сбой питания от электросети! Ожидание (90 секунд) подтверждения отсутствия питания от электросети." in the sql table "журнал_работы_приложения".
    pub fn log_power_failure() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Произошел сбой питания от электросети! Ожидание (90 секунд) подтверждения отсутствия питания от электросети.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Питание от электросети есть." in the sql table "журнал_работы_приложения".
    pub fn log_power_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Питание от электросети есть.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Подтверждение отсутствия питания от электросети." in the sql table "журнал_работы_приложения".
    pub fn log_power_failure_confirmed() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Подтверждение отсутствия питания от электросети.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Успешный старт генератора." in the sql table "журнал_работы_приложения".
    pub fn log_start_generator_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Успешный старт генератора.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Сбой старта генератора!" in the sql table "журнал_работы_приложения".
    pub fn log_start_generator_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Сбой старта генератора!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Питание от электросети восстановлено." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Питание от электросети восстановлено.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Генератор исправен. Генератор работает." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored_generator_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Генератор исправен. Генератор работает.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Генератор неисправен. Генератор не работает." in the sql table "журнал_работы_приложения".
    pub fn log_power_restored_generator_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Генератор неисправен. Генератор не работает.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Питание от электросети еще не было восстановлено, после отключения." in the sql table "журнал_работы_приложения".
    pub fn log_power_dont_restored() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Питание от электросети еще не было восстановлено, после отключения.";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_start_generator_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_start_generator_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_power_restored_generator_ok() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913" in the sql table "журнал_работы_приложения".
    pub fn log_send_sms_power_restored_generator_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Server error! Ошибка! SMS уведомление не было отправлено!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Http request status error! Ошибка! SMS уведомление не было отправлено!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Ошибка! Связь OPC сервера с ПЛК отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_plc_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Ошибка! Связь OPC сервера с ПЛК отсутствует!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Records log "Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!" in the sql table "журнал_работы_приложения".
    pub fn log_opc_err() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
        let event = "Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!";
        client
            .execute(
                "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
                &[&event],
            )
            ?;

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

    /// Timer for delaying the operation of the stream in order to wait
    /// for confirmation of a power failure from the mains.
    fn timer_90sec() {
        let timer = timer::Timer::new();
        let (tx, rx) = channel();

        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(90), move || {
            tx.send(()).unwrap();
            let _ignored = tx.send(());
        });

        rx.recv().unwrap();
    }

    /// Main spawn - the function for detecting a power failure from the mains/restoring power from the mains,
    /// successful start of the generator, failure of the generator start, and notifications about these events.
    /// Additional spawn - the function of determining the serviceability/malfunction of the generator
    /// and notifying about it by SMS using the gateway API.
    pub fn ats_state() -> Result<(), Error> {
        let mut client =
            Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
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
                                    log_power_failure();
                                    timer_90sec();
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
                                                                log_power_failure_confirmed();
                                                                // Checking internet access.
                                                                if check(None).is_ok() {
                                                                    println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                    if powersupply.start_generator == 1 {
                                                                        println!("Успешный старт генератора");
                                                                        event_power_failure_start_generator_ok();
                                                                        log_start_generator_ok();
                                                                        // Executing an http get request to the SMS gateway provider.
                                                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Успешный+старт+генератора.").unwrap();
                                                                        if resp.status().is_success() {
                                                                            println!("Http запрос выполнен успешно");
                                                                            println!("Отправлено SMS сообщение: /Сбой питания от электросети. Успешный старт генератора./ на номер +79139402913");
                                                                            log_send_sms_start_generator_ok();
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
                                                                        println!("Сбой старта генератора!");
                                                                        event_power_failure_start_generator_err();
                                                                        log_start_generator_err();
                                                                        // Executing an http get request to the SMS gateway provider.
                                                                        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Сбой+старта+генератора.").unwrap();
                                                                        if resp.status().is_success() {
                                                                            println!("Http запрос выполнен успешно");
                                                                            println!("Отправлено SMS сообщение: /Сбой питания от электросети. Сбой старта генератора./ на номер +79139402913");
                                                                            log_send_sms_start_generator_err();
                                                                        } else if resp.status().is_server_error() {
                                                                            println!("Server error!");
                                                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                                                            log_server_err();
                                                                        } else {
                                                                            println!("Status http request: {}", resp.status());
                                                                            println!("Ошибка! SMS уведомление не было отправлено!");
                                                                            log_request_status_err();
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
                                                                                                    log_power_restored();
                                                                                                    // Checking internet access.
                                                                                                    if check(None).is_ok() {
                                                                                                        println!("Выполнение http запроса поставщику услуг SMS оповещения");
                                                                                                        if powersupply.generator_work == 1 {
                                                                                                            println!("Генератор исправен. Генератор работает");
                                                                                                            event_power_restored_generator_work_ok();
                                                                                                            log_power_restored_generator_ok();
                                                                                                            // Executing an http get request to the SMS gateway provider.
                                                                                                            let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Питание+от+электросети+восстановлено.+Генератор+исправен.+Генератор+работает.").unwrap();
                                                                                                            if resp.status().is_success() {
                                                                                                                println!("Http запрос выполнен успешно");
                                                                                                                println!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор исправен. Генератор работает./ на номер +79139402913");
                                                                                                                log_send_sms_power_restored_generator_ok();
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
                                                                                                            println!("Генератор неисправен. Генератор не работает");
                                                                                                            event_power_restored_generator_work_err();
                                                                                                            log_power_restored_generator_err();
                                                                                                            // Executing an http get request to the SMS gateway provider.
                                                                                                            let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Питание+от+электросети+восстановлено.+Генератор+неисправен.+Генератор+не+работает.").unwrap();
                                                                                                            if resp.status().is_success() {
                                                                                                                println!("Http запрос выполнен успешно");
                                                                                                                println!("Отправлено SMS сообщение: /Питание от электросети восстановлено. Генератор неисправен. Генератор не работает./ на номер +79139402913");
                                                                                                                log_send_sms_power_restored_generator_err();
                                                                                                            } else if resp.status().is_server_error() {
                                                                                                                println!("Server error!");
                                                                                                                println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                                log_server_err();
                                                                                                            } else {
                                                                                                                println!("Status http request: {}", resp.status());
                                                                                                                println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                                log_request_status_err();
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        println!("Ошибка! Доступ к интернету отсутствует!");
                                                                                                        println!("Ошибка! Http запрос не был выполнен!");
                                                                                                        println!("Ошибка! SMS уведомление не было отправлено!");
                                                                                                        log_internet_err();
                                                                                                    }
                                                                                                    break 'inner;
                                                                                                } else {
                                                                                                    println!("Питание от электросети еще не было восстановлено, после отключения");
                                                                                                    log_power_dont_restored();
                                                                                                    timer_3sec();
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            println!("Ошибка! Связь OPC сервера с ПЛК отсутствует!");
                                                                                            log_plc_err();
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    println!("Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!");
                                                                                    log_opc_err();
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("Ошибка! Доступ к сети интернет осутствует!");
                                                                    println!("Ошибка! Http запрос не был выполнен!");
                                                                    println!("Ошибка! SMS уведомление не было отправлено!");
                                                                    log_internet_err();
                                                                }
                                                            } else {
                                                                println!("Питание от электросети восстановлено");
                                                                log_power_restored();
                                                            }
                                                        }
                                                    } else {
                                                        println!("Ошибка! Связь OPC сервера с ПЛК отсутствует!");
                                                        log_plc_err();
                                                    }
                                                }
                                            } else {
                                                println!("Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!");
                                                log_opc_err();
                                            }
                                        }
                                    }
                                } else {
                                    println!("Питание от электросети есть");
                                    event_power_ok();
                                    log_power_ok();
                                }
                            }
                        } else {
                            println!("Ошибка! Связь OPC сервера с ПЛК отсутствует!");
                            log_plc_err();
                        }
                    }
                } else {
                    println!("Ошибка! Связь СУБД PostgreSQL с OPC сервером отсутствует!");
                    log_opc_err();
                }
            }
        }
        Ok(())
    }
}
