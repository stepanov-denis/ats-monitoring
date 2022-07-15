pub mod avr_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(
        client: & mut TcpClient,
    ) {
        let mains_power_supply_response = client.read_input_registers(00002, 1);
        info!(
            "response reading_input_registers() mains_power_supply: {:?}",
            mains_power_supply_response
        );

        let start_generator_response = client.read_input_registers(00003, 1);
        info!(
            "response reading_input_registers() start_generator: {:?}",
            start_generator_response
        );

        let generator_faulty_response = client.read_input_registers(00005, 1);
        info!(
            "response reading_input_registers() generator_faulty: {:?}",
            generator_faulty_response
        );

        let generator_work_response = client.read_input_registers(00006, 1);
        info!(
            "response reading_input_registers() generator_work: {:?}",
            generator_work_response
        );

        let connection_response = client.read_input_registers(00019, 1);
        info!(
            "response reading_input_registers() connection: {:?}",
            connection_response
        );

        let load_response = client.read_input_registers(00004, 1);
        info!(
            "response reading_input_registers() load: {:?}",
            load_response
        );

        if mains_power_supply_response.len() == 1
            && start_generator_response.len() == 1
            && generator_faulty_response.len() == 1
            && generator_work_response.len() == 1
            && connection_response.len() == 1
            && load_response.len() == 1
        {
            match crate::psql::postgresql::insert_ats(
                mains_power_supply_response[0] as i32,
                start_generator_response[0] as i32,
                generator_faulty_response[0] as i32,
                generator_work_response[0] as i32,
                connection_response[0] as i32
            ) {
                Ok(_) => info!("insert_input_registers_ats(): ok"),
                Err(e) => info!("{}", e)
            }

            match crate::psql::postgresql::insert_generator_load(load_response[0] as i32) {
                Ok(_) => info!("insert_generator_load(): ok"),
                Err(e) => info!("{}", e)
            }
        } else {
            info!("error: not all values are transmitted to the app from the plc");
        }
    }

    /// Communication session with the PLC via Modbus TCP
    pub fn avr_control() {
        let mut client = TcpClient::new("10.54.52.201:502");
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
        let mut client = TcpClient::new("10.54.52.201:502");
        let result = client.connect();
        match result {
            Err(message) => {
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::alarm::info::event_err_connect_to_plc(&message);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                let connection_response = client.read_input_registers(00019, 1);
                info!("response reading_connection(): {:?}", connection_response);
                client.disconnect();
                match connection_response.len() {
                    1 => match connection_response[0] {
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
