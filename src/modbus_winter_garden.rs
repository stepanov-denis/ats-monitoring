pub mod winter_garden {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    fn log_error_connection(message: &str) {
        let event = format!("error: there is no connection between the app and the plc, {}",
        message);
        info!("{}", event);
        // Records event to the SQL table 'app_log'.
        match crate::psql::postgresql::insert_event(&event) {
            Ok(_) => info!("insert_event(): {}", event),
            Err(e) => info!("{}", e)
        }
    }

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    fn reading_input_registers(client: &mut TcpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
        let phyto_lighting_1 = client.read_input_registers(00007, 1);
        info!(
            "Response IR phyto_lighting_1: {:?}",
            phyto_lighting_1
        );

        let phyto_lighting_2 = client.read_input_registers(00008, 1);
        info!(
            "Response IR phyto_lighting_2: {:?}",
            phyto_lighting_2
        );

        let phyto_lighting_3 = client.read_input_registers(00009, 1);
        info!(
            "Response IR phyto_lighting_3: {:?}",
            phyto_lighting_3
        );

        let phyto_lighting_4 = client.read_input_registers(00010, 1);
        info!(
            "Response IR phyto_lighting_4: {:?}",
            phyto_lighting_4
        );

        let fan = client.read_input_registers(00011, 1);
        info!("Response IR fan: {:?}", fan);

        let automatic_watering_1 = client.read_input_registers(00012, 1);
        info!(
            "Response IR automatic_watering_1: {:?}",
            automatic_watering_1
        );

        let automatic_watering_2 = client.read_input_registers(00013, 1);
        info!(
            "Response IR automatic_watering_2: {:?}",
            automatic_watering_2
        );

        let automatic_watering_3 = client.read_input_registers(00014, 1);
        info!(
            "Response IR automatic_watering_3: {:?}",
            automatic_watering_3
        );

        let temperature_indoor = client.read_input_registers(00015, 1);
        info!(
            "Response IR temperature_indoor: {:?}",
            temperature_indoor
        );

        let humidity_indoor = client.read_input_registers(00016, 1);
        info!(
            "Response IR humidity_indoor: {:?}",
            humidity_indoor
        );

        let illumination_indoor = client.read_input_registers(00017, 1);
        info!(
            "Response IR illumination_indoor: {:?}",
            illumination_indoor
        );

        let illumination_outdoor = client.read_input_registers(00018, 1);
        info!(
            "Response IR illumination_outdoor: {:?}",
            illumination_outdoor
        );

        if phyto_lighting_1.len() == 1
            && phyto_lighting_2.len() == 1
            && phyto_lighting_3.len() == 1
            && phyto_lighting_4.len() == 1
            && fan.len() == 1
            && automatic_watering_1.len() == 1
            && automatic_watering_2.len() == 1
            && automatic_watering_3.len() == 1
            && temperature_indoor.len() == 1
            && humidity_indoor.len() == 1
            && illumination_indoor.len() == 1
            && illumination_outdoor.len() == 1
        {
            match crate::psql::postgresql::insert_winter_garden(
                phyto_lighting_1[0] as i32,
                phyto_lighting_2[0] as i32,
                phyto_lighting_3[0] as i32,
                phyto_lighting_4[0] as i32,
                fan[0] as i32,
                automatic_watering_1[0] as i32,
                automatic_watering_2[0] as i32,
                automatic_watering_3[0] as i32,
                temperature_indoor[0] as i32,
                humidity_indoor[0] as i32,
                illumination_indoor[0] as i32,
                illumination_outdoor[0] as i32)
            {
                Ok(_) => info!("crate::psql::postgresql::insert_winter_garden(): ok"),
                Err(e) => info!("{}", e)
            }
        } else {
            info!("error: not all values are transmitted to the app from the plc")
        }
        Ok(())
    }

    /// Communication session with the PLC via Modbus TCP.
    pub fn winter_garden() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut client = TcpClient::new("10.54.52.201:502");
        let result = client.connect();
        match result {
            Err(message) => {
                log_error_connection(&message);
            }
            Ok(_) => {
                info!("app communication with plc: ok");
                // Reading variable values from the PLC "trim5" via Modbus TCP
                // and writing the obtained values to the PostgreSQL DBMS.
                match reading_input_registers(&mut client) {
                    Ok(_) => info!("reading_input_registers(): ok"),
                    Err(e) => info!("{}", e),
                }
                client.disconnect();
            }
        }
        Ok(())
    }
}
