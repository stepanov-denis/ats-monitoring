pub mod winter_garden_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use std::error::Error;

    /// Data structure for variables of the automatic winter garden management system.
    pub struct WinterGarden {
        pub phyto_lighting_1: i32,
        pub phyto_lighting_2: i32,
        pub phyto_lighting_3: i32,
        pub phyto_lighting_4: i32,
        pub fan: i32,
        pub automatic_watering_1: i32,
        pub automatic_watering_2: i32,
        pub automatic_watering_3: i32,
        pub temperature_indoor: i32,
        pub humidity_indoor: i32,
        pub illumination_indoor: i32,
        pub illumination_outdoor: i32,
    }

    /// Reading the value of a variable from modbus registers.
    fn read(client: &mut TcpClient, adress: &str, quantity: u16) -> Vec<u16> {
        client.read_input_registers(
            crate::read_env::env::read_u16(adress).unwrap_or_default(),
            quantity,
        )
    }

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    fn reading_input_registers(client: &mut TcpClient) -> Result<(), Box<dyn Error + Send + Sync>> {
        let phyto_lighting_1: Vec<u16> = read(client, "PHYTO_LIGHTING_1", 1);
        info!("Response IR phyto_lighting_1: {:?}", phyto_lighting_1);

        let phyto_lighting_2: Vec<u16> = read(client, "PHYTO_LIGHTING_2", 1);
        info!("Response IR phyto_lighting_2: {:?}", phyto_lighting_2);

        let phyto_lighting_3: Vec<u16> = read(client, "PHYTO_LIGHTING_3", 1);
        info!("Response IR phyto_lighting_3: {:?}", phyto_lighting_3);

        let phyto_lighting_4: Vec<u16> = read(client, "PHYTO_LIGHTING_4", 1);
        info!("Response IR phyto_lighting_4: {:?}", phyto_lighting_4);

        let fan: Vec<u16> = read(client, "FAN", 1);
        info!("Response IR fan: {:?}", fan);

        let automatic_watering_1: Vec<u16> = read(client, "AUTOMATIC_WATERING_1", 1);
        info!(
            "Response IR automatic_watering_1: {:?}",
            automatic_watering_1
        );

        let automatic_watering_2: Vec<u16> = read(client, "AUTOMATIC_WATERING_2", 1);
        info!(
            "Response IR automatic_watering_2: {:?}",
            automatic_watering_2
        );

        let automatic_watering_3: Vec<u16> = read(client, "AUTOMATIC_WATERING_3", 1);
        info!(
            "Response IR automatic_watering_3: {:?}",
            automatic_watering_3
        );

        let temperature_indoor: Vec<u16> = read(client, "TEMPERATURE_INDOOR", 1);
        info!("Response IR temperature_indoor: {:?}", temperature_indoor);

        let humidity_indoor: Vec<u16> = read(client, "HUMIDITY_INDOOR", 1);
        info!("Response IR humidity_indoor: {:?}", humidity_indoor);

        let illumination_indoor: Vec<u16> = read(client, "ILLUMINATION_INDOOR", 1);
        info!("Response IR illumination_indoor: {:?}", illumination_indoor);

        let illumination_outdoor: Vec<u16> = read(client, "ILLUMINATION_OUTDOOR", 1);
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
            let winter_garden: WinterGarden = WinterGarden {
                phyto_lighting_1: phyto_lighting_1[0] as i32,
                phyto_lighting_2: phyto_lighting_2[0] as i32,
                phyto_lighting_3: phyto_lighting_3[0] as i32,
                phyto_lighting_4: phyto_lighting_4[0] as i32,
                fan: fan[0] as i32,
                automatic_watering_1: automatic_watering_1[0] as i32,
                automatic_watering_2: automatic_watering_2[0] as i32,
                automatic_watering_3: automatic_watering_3[0] as i32,
                temperature_indoor: temperature_indoor[0] as i32,
                humidity_indoor: humidity_indoor[0] as i32,
                illumination_indoor: illumination_indoor[0] as i32,
                illumination_outdoor: illumination_outdoor[0] as i32,
            };

            match crate::psql::postgresql::insert_winter_garden(winter_garden) {
                Ok(_) => info!("insert_winter_garden(): ok"),
                Err(e) => {
                    let message = format!("insert_winter_garden() error: {}", e);
                    info!("{}", message);
                    // Sending telegram notification.
                    crate::tg::api::send_notification(&message);
                }
            }
        } else {
            let event = "winter_garden control error: not all values are transmitted to the app from the plc";
            // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
            crate::logger::log::record(event);
        }
        Ok(())
    }

    /// Communication session with the PLC via Modbus TCP.
    fn modbus_client() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut client =
            TcpClient::new(&crate::read_env::env::read_str("IP_TRIM5").unwrap_or_default());
        let result = client.connect();
        match result {
            Err(message) => {
                let event = format!("winter_garden() error: {}", message);
                // Create event "app connection error to PLC".
                // and records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::event_err_connect_to_plc(&event);
                // Sending telegram notification.
                crate::tg::api::send_notification(&event);
            }
            Ok(_) => {
                let event = "app communication with plc: ok";
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(event);
                // Reading variable values from the PLC "trim5" via Modbus TCP
                // and writing the obtained values to the PostgreSQL DBMS.
                match reading_input_registers(&mut client) {
                    Ok(_) => {
                        let event = "reading_input_registers(): ok";
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(event);
                    }
                    Err(e) => {
                        let event = format!(
                            "ats_control::winter_garden() reading_input_registers() error: {}",
                            e
                        );
                        // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                        crate::logger::log::record(&event);
                        // Sending telegram notification.
                        crate::tg::api::send_notification(&event);
                    }
                }
                client.disconnect();
            }
        }
        Ok(())
    }

    pub fn winter_garden() {
        match modbus_client() {
            Ok(_) => info!("winter_garden(): ok"),
            Err(e) => {
                let event = format!("winter_garden() error: {}", e);
                // Records the event to the SQL table 'app_log' and outputs it to info! env_logger.
                crate::logger::log::record(&event);
                // Sending telegram notification.
                crate::tg::api::send_notification(&event);
            }
        }
    }
}
