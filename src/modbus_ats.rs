pub mod avr_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS
    pub fn reading_input_registers(client: &mut TcpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
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
        info!("response reading_input_registers() generator_work: {:?}", generator_work_response);

        let connection_response = client.read_input_registers(00019, 1);
        info!("response reading_input_registers() connection: {:?}", connection_response);

        let load_response = client.read_input_registers(00004, 1);
        info!("response reading_input_registers() load: {:?}", load_response);

        if mains_power_supply_response.len() == 1
            && start_generator_response.len() == 1
            && generator_faulty_response.len() == 1
            && generator_work_response.len() == 1
            && connection_response.len() == 1
            && load_response.len() == 1
        {
            if crate::psql::postgresql::insert_input_registers_ats(
                mains_power_supply_response[0] as i32,
                start_generator_response[0] as i32,
                generator_faulty_response[0] as i32,
                generator_work_response[0] as i32,
                connection_response[0] as i32).is_ok() {
                    info!("insert_input_registers_ats(): ok");
                } else {
                    info!("insert_input_registers_ats(): error");
                }

            if crate::psql::postgresql::insert_generator_load(load_response[0] as i32).is_ok() {
                info!("insert_generator_load(): ok");
            } else {
                info!("insert_generator_load(): error");
            }
        } else {
            info!("error: not all values are transmitted to the app from the plc");
        }
        Ok(())
    }

    /// Communication session with the PLC via Modbus TCP
    pub fn avr_control() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut client = TcpClient::new("10.54.52.201:502");
        let result = client.connect();
        match result {
            Err(message) => {
                info!(
                    "error: there is no connection between the app and the plc {}",
                    message
                );
                if crate::psql::postgresql::log_timeout_or_host_unreachable_modbus_ats().is_ok() {
                    info!("log_timeout_or_host_unreachable_modbus_ats(): ok");
                } else {
                    info!("log_timeout_or_host_unreachable_modbus_ats(): error");
                }
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                if reading_input_registers(&mut client).is_ok() {
                    info!("reading_input_registers(): ok");
                } else {
                    info!("reading_input_registers(): error");
                }
                client.disconnect();
            }
        }
        Ok(())
    }
}
