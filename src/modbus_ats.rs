pub mod ats_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    /// Data structure for variables of the automatic emergency reserve
    /// entry control system.
    pub struct Ats {
        pub mains_power_supply: i32,
        pub start_generator: i32,
        pub generator_faulty: i32,
        pub transmitted_work: i32,
        pub connection: i32,
    }

    /// Data structure for the load level variable connected to the generator.
    pub struct GeneratorLoad {
        pub load: i32,
    }

    /// Reading the value of a variable from modbus registers.
    fn read(client: &mut TcpClient, adress: &str, quantity: u16) -> Vec<u16> {
        client.read_input_registers(
            crate::read_env::env::read_u16(adress).unwrap_or_default(),
            quantity,
        )
    }

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(client: &mut TcpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mains_power_supply: Vec<u16> = read(client, "MAINS_POWER_SUPPLY", 1);
        info!(
            "response reading_input_registers() mains_power_supply: {:?}",
            mains_power_supply
        );

        let start_generator: Vec<u16> = read(client, "START_GENERATOR", 1);
        info!(
            "response reading_input_registers() start_generator: {:?}",
            start_generator
        );

        let generator_faulty: Vec<u16> = read(client, "GENERATOR_FAULTY", 1);
        info!(
            "response reading_input_registers() generator_faulty: {:?}",
            generator_faulty
        );

        let transmitted_work: Vec<u16> = read(client, "TRANSMITTED_WORK", 1);
        info!(
            "response reading_input_registers() generator_work: {:?}",
            transmitted_work
        );

        let connection: Vec<u16> = read(client, "CONNECTION", 1);
        info!(
            "response reading_input_registers() connection: {:?}",
            connection
        );

        let load: Vec<u16> = read(client, "LOAD", 1);
        info!("response reading_input_registers() load: {:?}", load);

        if mains_power_supply.len() == 1
            && start_generator.len() == 1
            && generator_faulty.len() == 1
            && transmitted_work.len() == 1
            && connection.len() == 1
            && load.len() == 1
        {
            let ats: Ats = Ats {
                mains_power_supply: mains_power_supply[0] as i32,
                start_generator: start_generator[0] as i32,
                generator_faulty: generator_faulty[0] as i32,
                transmitted_work: transmitted_work[0] as i32,
                connection: connection[0] as i32,
            };

            let generator_load: GeneratorLoad = GeneratorLoad {
                load: load[0] as i32,
            };

            match crate::psql::postgresql::insert_ats(ats) {
                Ok(_) => info!("insert_input_registers_ats(): ok"),
                Err(e) => info!("{}", e),
            }

            match crate::psql::postgresql::insert_generator_load(generator_load) {
                Ok(_) => info!("insert_generator_load(): ok"),
                Err(e) => info!("{}", e),
            }
        } else {
            let event = "ats_control::ats() reading_input_registers() error: not all values are transmitted to the app from the plc";
            // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
            crate::logger::log::record(event);
        }
        Ok(())
    }

    /// Communication session with the PLC via Modbus TCP
    pub fn ats() {
        let mut client =
            TcpClient::new(&crate::read_env::env::read_str("IP_TRIM5").unwrap_or_default());
        let result = client.connect();
        match result {
            Err(message) => {
                let event = format!("ats() error: {}", message);
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::event_err_connect_to_plc(&event);
            }
            Ok(_) => {
                let event = "app communication with plc: ok";
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Reading variable values from the PLC "trim5" via Modbus TCP
                // and writing the obtained values to the PostgreSQL DBMS.
                match reading_input_registers(&mut client) {
                    Ok(_) => {
                        let event = "reading_input_registers(): ok";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                    }
                    Err(e) => {
                        let event = format!("ats_control::ats() reading_input_registers() error: {}", e);
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                    }
                }
                client.disconnect();
            }
        }
    }

    /// Reading the value of the "connection" variable from the TRIM5 PLC via Modbus TCP
    /// to check the connection of the app to the PLC.
    pub fn reading_connection() -> Option<bool> {
        let mut client =
            TcpClient::new(&crate::read_env::env::read_str("IP_TRIM5").unwrap_or_default());
        let result = client.connect();
        match result {
            Err(message) => {
                let event = format!("reading_connection() error: {}", message);
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::event_err_connect_to_plc(&event);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                let connection = read(&mut client, "CONNECTION", 1);
                info!("response reading_connection(): {:?}", connection);
                client.disconnect();
                match connection.len() {
                    1 => match connection[0] {
                        1 => return Some(true),
                        _ => return Some(false)
                    }
                    _ => {
                        let event = "reading_connection() error: the value is not transmitted to the app from the plc";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                    }
                }
            }
        }
        Some(false)
    }
}
