pub mod avr_control {
    extern crate modbus_iiot;
    use modbus_iiot::tcp::master::TcpClient;
    use modbus_iiot::tcp::masteraccess::MasterAccess;
    use postgres::{Client, NoTls};

    /// Reading variable values from the PLC "trim5" via Modbus TCP and writing the obtained values to the PostgreSQL DBMS.
    pub fn reading_input_registers(client: &mut TcpClient) {
        let mains_power_supply_response = client.read_input_registers(00002, 1);
        println!(
            "Response IR mains_power_supply: {:?}",
            mains_power_supply_response
        );

        let start_generator_response = client.read_input_registers(00003, 1);
        println!(
            "Response IR start_generator: {:?}",
            start_generator_response
        );

        let generator_faulty_response = client.read_input_registers(00005, 1);
        println!(
            "Response IR generator_faulty: {:?}",
            generator_faulty_response
        );

        let generator_work_response = client.read_input_registers(00006, 1);
        println!("Response IR generator_work: {:?}", generator_work_response);

        let connection_response = client.read_input_registers(00019, 1);
        println!("Response IR connection: {:?}", connection_response);

        let load_response = client.read_input_registers(00004, 1);
        println!("Response IR load: {:?}", load_response);

        let mut client =
            Client::connect("postgresql://stepanov:postgres@localhost/postgres", NoTls).unwrap();

        let mains_power_supply: i32 = mains_power_supply_response[0] as i32;
        let start_generator: i32 = start_generator_response[0] as i32;
        let generator_faulty: i32 = generator_faulty_response[0] as i32;
        let generator_work: i32 = generator_work_response[0] as i32;
        let connection: i32 = connection_response[0] as i32;
        client.execute(
            "INSERT INTO avr_control_insert (mains_power_supply, start_generator, generator_faulty, generator_work, connection) VALUES ($1, $2, $3, $4, $5)",
            &[&mains_power_supply, &start_generator, &generator_faulty, &generator_work, &connection],
        ).unwrap();

        for row in client.query("SELECT mains_power_supply, start_generator, generator_faulty, generator_work, connection FROM avr_control_insert ORDER BY mark DESC limit 1", &[]).unwrap() {
            let mains_power_supply: i32 = row.get(0);
            let start_generator: i32 = row.get(1);
            let generator_faulty: i32 = row.get(2);
            let generator_work: i32 = row.get(3);
            let connection: i32 = row.get(4);
            println!(
                "Считаны из ПЛК и записаны в табл. avr_control_insert следующие значения: mains_power_supply: {}, start_generator: {}, generator_faulty: {}, generator_work: {}, connection: {}",
                mains_power_supply, start_generator, generator_faulty, generator_work, connection);
        }

        let load: i32 = load_response[0] as i32;
        client
            .execute(
                "INSERT INTO нагрузка_на_генератор (нагрузка) VALUES ($1)",
                &[&load],
            )
            .unwrap();

        for row in client
            .query(
                "SELECT нагрузка FROM нагрузка_на_генератор ORDER BY время_и_дата DESC limit 1",
                &[],
            )
            .unwrap()
        {
            let load: i32 = row.get(0);
            println!(
                "Считываны из ПЛК и записаны в табл. нагрузка_на_генератор следующие значения: load: {}",
                load);
        }
    }

    /// Communication session with the PLC via Modbus TCP.
    pub fn avr_control_insert() {
        let mut client = TcpClient::new("10.54.52.201:502");
        let result = client.connect();
        match result {
            Err(message) => println!(
                "Ошибка! Связь ПЛК с модулем modbus_ats отсутствует! {}",
                message
            ),
            Ok(_) => {
                println!("Связь ПЛК с модулем modbus_ats: Ok");
                reading_input_registers(&mut client);

                client.disconnect();
            }
        }
    }
}
