pub mod winter_garden {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(client: &mut TcpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
        let phyto_lighting_1_response = client.read_input_registers(00007, 1);
        info!(
            "Response IR phyto_lighting_1: {:?}",
            phyto_lighting_1_response
        );

        let phyto_lighting_2_response = client.read_input_registers(00008, 1);
        info!(
            "Response IR phyto_lighting_2: {:?}",
            phyto_lighting_2_response
        );

        let phyto_lighting_3_response = client.read_input_registers(00009, 1);
        info!(
            "Response IR phyto_lighting_3: {:?}",
            phyto_lighting_3_response
        );

        let phyto_lighting_4_response = client.read_input_registers(00010, 1);
        info!(
            "Response IR phyto_lighting_4: {:?}",
            phyto_lighting_4_response
        );

        let fan_response = client.read_input_registers(00011, 1);
        info!("Response IR fan: {:?}", fan_response);

        let automatic_watering_1_response = client.read_input_registers(00012, 1);
        info!(
            "Response IR automatic_watering_1: {:?}",
            automatic_watering_1_response
        );

        let automatic_watering_2_response = client.read_input_registers(00013, 1);
        info!(
            "Response IR automatic_watering_2: {:?}",
            automatic_watering_2_response
        );

        let automatic_watering_3_response = client.read_input_registers(00014, 1);
        info!(
            "Response IR automatic_watering_3: {:?}",
            automatic_watering_3_response
        );

        let temperature_indoor_response = client.read_input_registers(00015, 1);
        info!(
            "Response IR temperature_indoor: {:?}",
            temperature_indoor_response
        );

        let humidity_indoor_response = client.read_input_registers(00016, 1);
        info!(
            "Response IR humidity_indoor: {:?}",
            humidity_indoor_response
        );

        let illumination_indoor_response = client.read_input_registers(00017, 1);
        info!(
            "Response IR illumination_indoor: {:?}",
            illumination_indoor_response
        );

        let illumination_outdoor_response = client.read_input_registers(00018, 1);
        info!(
            "Response IR illumination_outdoor: {:?}",
            illumination_outdoor_response
        );

        if phyto_lighting_1_response.len() == 1
            && phyto_lighting_2_response.len() == 1
            && phyto_lighting_3_response.len() == 1
            && phyto_lighting_4_response.len() == 1
            && fan_response.len() == 1
            && automatic_watering_1_response.len() == 1
            && automatic_watering_2_response.len() == 1
            && automatic_watering_3_response.len() == 1
            && temperature_indoor_response.len() == 1
            && humidity_indoor_response.len() == 1
            && illumination_indoor_response.len() == 1
            && illumination_outdoor_response.len() == 1
        {
            match crate::psql::postgresql::insert_winter_garden(
                phyto_lighting_1_response[0] as i32,
                phyto_lighting_2_response[0] as i32,
                phyto_lighting_3_response[0] as i32,
                phyto_lighting_4_response[0] as i32,
                fan_response[0] as i32,
                automatic_watering_1_response[0] as i32,
                automatic_watering_2_response[0] as i32,
                automatic_watering_3_response[0] as i32,
                temperature_indoor_response[0] as i32,
                humidity_indoor_response[0] as i32,
                illumination_indoor_response[0] as i32,
                illumination_outdoor_response[0] as i32)
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
                info!(
                    "error: there is no connection between the app and the plc, {}",
                    message
                );
                // Records log
                // "Ошибка! Связь ПЛК с модулем modbus_ats отсутствует!" in the sql table "журнал_работы_приложения".
                match crate::psql::postgresql::log_timeout_or_host_unreachable_modbus_ats() {
                    Ok(_) => info!("crate::psql::postgresql::log_timeout_or_host_unreachable_modbus_ats(): ok"),
                    Err(e) => info!("{}", e)
                }
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
