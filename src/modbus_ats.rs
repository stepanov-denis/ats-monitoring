pub mod avr_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;

    fn read(client: &mut TcpClient, adress: &str, quantity: u16) -> Vec<u16> {
        client.read_input_registers(crate::read_env::env::read_u16(adress).unwrap_or_default(), quantity)
    }

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(client: &mut TcpClient) {
        let mains_power_supply = read(client, "MAINS_POWER_SUPPLY", 1);
        info!(
            "response reading_input_registers() mains_power_supply: {:?}",
            mains_power_supply
        );

        let start_generator = read(client, "START_GENERATOR", 1);
        info!(
            "response reading_input_registers() start_generator: {:?}",
            start_generator
        );

        let generator_faulty = read(client, "GENERATOR_FAULTY", 1);
        info!(
            "response reading_input_registers() generator_faulty: {:?}",
            generator_faulty
        );

        let generator_work = read(client, "GENERATOR_WORK", 1);
        info!(
            "response reading_input_registers() generator_work: {:?}",
            generator_work
        );

        let connection = read(client, "CONNECTION", 1);
        info!(
            "response reading_input_registers() connection: {:?}",
            connection
        );

        let load = read(client, "LOAD", 1);
        info!(
            "response reading_input_registers() load: {:?}",
            load
        );

        if mains_power_supply.len() == 1
            && start_generator.len() == 1
            && generator_faulty.len() == 1
            && generator_work.len() == 1
            && connection.len() == 1
            && load.len() == 1
        {
            match crate::psql::postgresql::insert_ats(
                mains_power_supply[0] as i32,
                start_generator[0] as i32,
                generator_faulty[0] as i32,
                generator_work[0] as i32,
                connection[0] as i32,
            ) {
                Ok(_) => info!("insert_input_registers_ats(): ok"),
                Err(e) => info!("{}", e),
            }

            match crate::psql::postgresql::insert_generator_load(load[0] as i32) {
                Ok(_) => info!("insert_generator_load(): ok"),
                Err(e) => info!("{}", e),
            }
        } else {
            info!("error: not all values are transmitted to the app from the plc");
        }
    }

    /// Communication session with the PLC via Modbus TCP
    pub fn avr_control() {
        let mut client = TcpClient::new(&crate::read_env::env::read_str("IP_TRIM5").unwrap_or_default());
        let result = client.connect();
        match result {
            Err(message) => {
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::alarm::info::event_err_connect_to_plc(&message);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                // Reading variable values from the PLC "trim5" via Modbus TCP
                // and writing the obtained values to the PostgreSQL DBMS.
                reading_input_registers(&mut client);
                client.disconnect();
            }
        }
    }

    /// Reading the value of the "connection" variable from the TRIM5 PLC via Modbus TCP
    /// to check the connection of the app to the PLC.
    pub fn reading_connection() -> Option<bool> {
        let mut client = TcpClient::new(&crate::read_env::env::read_str("IP_TRIM5").unwrap_or_default());
        let result = client.connect();
        match result {
            Err(message) => {
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::alarm::info::event_err_connect_to_plc(&message);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                let connection = read(&mut client, "IP_TRIM5", 1);
                info!("response reading_connection(): {:?}", connection);
                client.disconnect();
                match connection.len() {
                    1 => match connection[0] {
                        1 => return Some(true),
                        _ => return Some(false)
                    }
                    _ => info!("reading_connection() error: the value is not transmitted to the app from the plc")
                }
            }
        }
        Some(false)
    }
}
