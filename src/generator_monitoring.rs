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
    pub fn generator_state() {
        if crate::skydb::skytable::unix_sql() + 5.00 > crate::skydb::skytable::unix_sql_now() {
            // Сhecking the connection of the OPC server with the plc.
            if crate::skydb::skytable::plc_connect() == 1 {
                // Request for the health status of the generator.
                println!("Запрос работоспособности генератора в режиме трансляции питания от электросети");
                println!(
                    "response from PostgreSQL: generator_faulty = {}",
                    crate::skydb::skytable::generator_faulty()
                );
                if crate::skydb::skytable::generator_faulty() == 1 {
                    println!("Авария! Генератор неисправен! Срочно произведите сервисные работы!");
                    crate::psql::postgresql::event_generator_work_err();
                    crate::psql::postgresql::log_generator_work_err();
                    // Checking for internet access.
                    if check(None).is_ok() {
                        println!("Выполнение http запроса поставщику услуг SMS оповещения");
                        // Executing an http get request to the SMS gateway provider.
                        crate::cycle::http::generator_monitoring_cycle();
                    } else {
                        println!("Ошибка! Доступ к интернету отсутствует!");
                        println!("Ошибка! Http запрос не был выполнен!");
                        println!("Ошибка! SMS уведомление не было отправлено!");
                        crate::psql::postgresql::log_internet_err();
                    }
                } else {
                    println!(
                        "Генератор в режиме трансляции питания от электросети работает исправно"
                    );
                    crate::psql::postgresql::event_generator_work_ok();
                    crate::psql::postgresql::log_generator_work_ok();
                }
            } else {
                println!("Ошибка! Связь Modbus клиента с ПЛК отсутствует!");
                crate::psql::postgresql::log_plc_err();
                timer_for_delay(3);
            }
        } else {
            println!("Ошибка! Связь СУБД PostgreSQL с Modbus клиентом отсутствует!");
            crate::psql::postgresql::log_opc_err();
            timer_for_delay(3);
        }
    }
}
