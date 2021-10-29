pub mod winter_garden {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use postgres::{Client, NoTls};

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(client: &mut TcpClient) {
        let phyto_lighting_1_response = client.read_input_registers(00007, 1);
        println!(
            "Response IR phyto_lighting_1: {:?}",
            phyto_lighting_1_response
        );

        let phyto_lighting_2_response = client.read_input_registers(00008, 1);
        println!(
            "Response IR phyto_lighting_2: {:?}",
            phyto_lighting_2_response
        );

        let phyto_lighting_3_response = client.read_input_registers(00009, 1);
        println!(
            "Response IR phyto_lighting_3: {:?}",
            phyto_lighting_3_response
        );

        let phyto_lighting_4_response = client.read_input_registers(00010, 1);
        println!(
            "Response IR phyto_lighting_4: {:?}",
            phyto_lighting_4_response
        );

        let fan_response = client.read_input_registers(00011, 1);
        println!("Response IR fan: {:?}", fan_response);

        let automatic_watering_1_response = client.read_input_registers(00012, 1);
        println!(
            "Response IR automatic_watering_1: {:?}",
            automatic_watering_1_response
        );

        let automatic_watering_2_response = client.read_input_registers(00013, 1);
        println!(
            "Response IR automatic_watering_2: {:?}",
            automatic_watering_2_response
        );

        let automatic_watering_3_response = client.read_input_registers(00014, 1);
        println!(
            "Response IR automatic_watering_3: {:?}",
            automatic_watering_3_response
        );

        let temperature_indoor_response = client.read_input_registers(00015, 1);
        println!(
            "Response IR temperature_indoor: {:?}",
            temperature_indoor_response
        );

        let humidity_indoor_response = client.read_input_registers(00016, 1);
        println!(
            "Response IR humidity_indoor: {:?}",
            humidity_indoor_response
        );

        let illumination_indoor_response = client.read_input_registers(00017, 1);
        println!(
            "Response IR illumination_indoor: {:?}",
            illumination_indoor_response
        );

        let illumination_outdoor_response = client.read_input_registers(00018, 1);
        println!(
            "Response IR illumination_outdoor: {:?}",
            illumination_outdoor_response
        );

        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls).unwrap();

        let phyto_lighting_1: i32 = phyto_lighting_1_response[0] as i32;
        let phyto_lighting_2: i32 = phyto_lighting_2_response[0] as i32;
        let phyto_lighting_3: i32 = phyto_lighting_3_response[0] as i32;
        let phyto_lighting_4: i32 = phyto_lighting_4_response[0] as i32;
        let fan: i32 = fan_response[0] as i32;
        let automatic_watering_1: i32 = automatic_watering_1_response[0] as i32;
        let automatic_watering_2: i32 = automatic_watering_2_response[0] as i32;
        let automatic_watering_3: i32 = automatic_watering_3_response[0] as i32;
        let temperature_indoor: i32 = temperature_indoor_response[0] as i32;
        let humidity_indoor: i32 = humidity_indoor_response[0] as i32;
        let illumination_indoor: i32 = illumination_indoor_response[0] as i32;
        let illumination_outdoor: i32 = illumination_outdoor_response[0] as i32;
        client.execute(
            "INSERT INTO зимний_сад (фитоосвещение_1, фитоосвещение_2, фитоосвещение_3, фитоосвещение_4, вентилятор, автополив_1, автополив_2, автополив_3, температура, влажность, освещенность_в_помещении, освещенность_на_улице) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[&phyto_lighting_1, &phyto_lighting_2, &phyto_lighting_3, &phyto_lighting_4, &fan, &automatic_watering_1, &automatic_watering_2, &automatic_watering_3, &temperature_indoor, &humidity_indoor, &illumination_indoor, &illumination_outdoor],
        ).unwrap();

        for row in client.query("SELECT фитоосвещение_1, фитоосвещение_2, фитоосвещение_3, фитоосвещение_4, вентилятор, автополив_1, автополив_2, автополив_3, температура, влажность, освещенность_в_помещении, освещенность_на_улице FROM зимний_сад ORDER BY время_и_дата DESC limit 1", &[]).unwrap() {
            let phyto_lighting_1: i32 = row.get(0);
            let phyto_lighting_2: i32 = row.get(1);
            let phyto_lighting_3: i32 = row.get(2);
            let phyto_lighting_4: i32 = row.get(3);
            let fan: i32 = row.get(4);
            let automatic_watering_1: i32 = row.get(5);
            let automatic_watering_2: i32 = row.get(6);
            let automatic_watering_3: i32 = row.get(7);
            let temperature_indoor: i32 = row.get(8);
            let humidity_indoor: i32 = row.get(9);
            let illumination_indoor: i32 = row.get(10);
            let illumination_outdoor: i32 = row.get(11);
            println!(
                "Считаны из ПЛК и записаны в табл. зимний_сад следующие значения: phyto_lighting_1: {}, phyto_lighting_2: {}, phyto_lighting_3: {}, phyto_lighting_4: {}, fan: {}, automatic_watering_1: {}, automatic_watering_2: {}, automatic_watering_3: {}, temperature_indoor: {}, humidity_indoor: {}, illumination_indoor: {}, illumination_outdoor: {}",
                phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor);
        }
    }

    /// Communication session with the PLC via Modbus TCP.
    pub fn winter_garden_insert() {
        let mut client = TcpClient::new("10.54.52.201:502");
        let result = client.connect();
        match result {
            Err(message) => println!(
                "Ошибка! Связь ПЛК с модулем modbus_winter_garden отсутствует! {}",
                message
            ),
            Ok(_) => {
                println!("Связь ПЛК с модулем modbus_winter_garden: Ok");
                reading_input_registers(&mut client);

                client.disconnect();
            }
        }
    }
}
