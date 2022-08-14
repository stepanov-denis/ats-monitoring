pub mod postgresql {
    use crate::modbus_ats::ats_control::Ats;
    use crate::modbus_ats::ats_control::GeneratorLoad;
    use crate::modbus_winter_garden::winter_garden_control::WinterGarden;
    use postgres::{Client, Error as PostgresError, NoTls};

    /// rRturns the database connection string.
    pub fn db_connect() -> String {
        // String::from("postgresql://postgres:mysecretpassword@postgresql:5432/postgres")
        let mut s = String::from("postgresql://");
        s.push_str(&crate::read_env::env::read_str("POSTGRES_USERNAME").unwrap_or_default());
        s.push(':');
        s.push_str(&crate::read_env::env::read_str("POSTGRES_PASSWORD").unwrap_or_default());
        s.push('@');
        s.push_str(&crate::read_env::env::read_str("POSTGRES_HOSTNAME").unwrap_or_default());
        s.push(':');
        s.push_str(&crate::read_env::env::read_str("POSTGRES_PORT").unwrap_or_default());
        s.push('/');
        s.push_str(&crate::read_env::env::read_str("POSTGRES_DB").unwrap_or_default());
        s
    }

    /// Set default transaction isolation level for database
    pub fn set_transaction_isolation() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "alter database postgres set default_transaction_isolation to serializable",
        )?;
        Ok(())
    }

    /// Create SQL table "avr_control".
    pub fn create_ats_control_table() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS ats_control (
                    id serial primary key,
                    mains_power_supply int not null,
                    start_generator int not null,
                    generator_faulty int not null,
                    transmitted_work int not null,
                    connection int not null,
                    date timestamptz default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "app_log".
    pub fn create_app_log_table() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS app_log (
                    id serial primary key,
                    event text not null,
                    date timestamp default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "winter_garden".
    pub fn create_winter_garden_table() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS winter_garden (
                    id serial primary key,
                    phyto_lighting_1 int not null,
                    phyto_lighting_2 int not null,
                    phyto_lighting_3 int not null,
                    phyto_lighting_4 int not null,
                    fan int not null,
                    automatic_watering_1 int not null,
                    automatic_watering_2 int not null,
                    automatic_watering_3 int not null,
                    temperature_indoor int not null,
                    humidity_indoor int not null,
                    illumination_indoor int not null,
                    illumination_outdoor int not null,
                    date timestamp default current_timestamp
                )
            ",
        )?;
        Ok(())
    }

    /// Create SQL table "generator_load".
    pub fn create_generator_load_table() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS generator_load (
                    id serial primary key,
                    load int not null,
                    date timestamp default current_timestamp
                
                )
            ",
        )?;
        Ok(())
    }

    pub fn create_tg_message_table() -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.batch_execute(
            "
                CREATE TABLE IF NOT EXISTS tg_message (
                    id serial primary key,
                    time int not null,
                    date timestamp default current_timestamp
                
                )
            ",
        )?;
        Ok(())
    }

    /// Records some event to the SQL table "app_log".
    pub fn insert_event(event: &str) -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.execute("INSERT INTO app_log (event) VALUES ($1)", &[&event])?;

        for row in client.query(
            "SELECT event, date FROM app_log ORDER BY date DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            info!("entry in the sql table 'app_log': {}", event);
        }
        Ok(())
    }

    /// Records the values of the variables of the automatic reserve to the SQL table "ats_control".
    pub fn insert_ats(ats: Ats) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute(
            "INSERT INTO ats_control (mains_power_supply, start_generator, generator_faulty, transmitted_work, connection) VALUES ($1, $2, $3, $4, $5)",
            &[&ats.mains_power_supply, &ats.start_generator, &ats.generator_faulty, &ats.transmitted_work, &ats.connection],
        )?;

        for row in client.query("SELECT mains_power_supply, start_generator, generator_faulty, transmitted_work, connection FROM ats_control ORDER BY date DESC limit 1", &[])? {
            let mains_power_supply: i32 = row.get(0);
            let start_generator: i32 = row.get(1);
            let generator_faulty: i32 = row.get(2);
            let transmitted_work: i32 = row.get(3);
            let connection: i32 = row.get(4);
            info!(
                "the following values are read from the plc and written to the 'ats_control' table: mains_power_supply: {}, start_generator: {}, generator_faulty: {}, transmitted_work: {}, connection: {}",
                mains_power_supply, start_generator, generator_faulty, transmitted_work, connection);
        }
        Ok(())
    }

    /// Records the values of the variables of the automatic winter garden management system
    /// to the SQL table "winter_garden".
    pub fn insert_winter_garden(winter_garden: WinterGarden) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute(
            "INSERT INTO winter_garden (phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[&winter_garden.phyto_lighting_1,
            &winter_garden.phyto_lighting_2,
            &winter_garden.phyto_lighting_3,
            &winter_garden.phyto_lighting_4,
            &winter_garden.fan,
            &winter_garden.automatic_watering_1,
            &winter_garden.automatic_watering_2,
            &winter_garden.automatic_watering_3,
            &winter_garden.temperature_indoor,
            &winter_garden.humidity_indoor,
            &winter_garden.illumination_indoor,
            &winter_garden.illumination_outdoor],
        )?;

        for row in client.query("SELECT phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor FROM winter_garden ORDER BY date DESC limit 1", &[])? {
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
            info!(
                "the following values are read from the plc and written to the table 'avr_control' зимний_сад: phyto_lighting_1: {}, phyto_lighting_2: {}, phyto_lighting_3: {}, phyto_lighting_4: {}, fan: {}, automatic_watering_1: {}, automatic_watering_2: {}, automatic_watering_3: {}, temperature_indoor: {}, humidity_indoor: {}, illumination_indoor: {}, illumination_outdoor: {}",
                phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor);
        }
        Ok(())
    }

    /// Records the value of the load level variable connected to the generator.
    pub fn insert_generator_load(generator_load: GeneratorLoad) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute(
            "INSERT INTO generator_load (load) VALUES ($1)",
            &[&generator_load.load],
        )?;

        for row in client.query(
            "SELECT load FROM generator_load ORDER BY date DESC limit 1",
            &[],
        )? {
            let load: i32 = row.get(0);
            info!(
                "the following values are read from the plc and written to the table 'generator_load' load: {}",
                load);
        }
        Ok(())
    }

    pub fn insert_message_time(message_time: i32) -> Result<(), PostgresError> {
        let mut client = Client::connect(&db_connect(), NoTls)?;
        client.execute("INSERT INTO tg_message (time) VALUES ($1)", &[&message_time])?;

        for row in client.query(
            "SELECT time, date FROM tg_message ORDER BY date DESC limit 1",
            &[],
        )? {
            let message_time: &str = row.get(0);

            info!("entry in the sql table 'tg_message': {}", message_time);
        }
        Ok(())
    }

    /// Getting generator_faulty value
    /// 0 - generator is working properly in the mode of electricity transmission from the power grid
    /// 1 - the generator does not work in the mode of transmission of electricity from the power grid
    /// 2 - the generator_faulty value is not 0 or 1.
    pub fn select_generator_faulty() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query(
            "SELECT generator_faulty FROM ats_control ORDER BY date DESC limit 1",
            &[],
        )?)
        .into_iter()
        .next()
        {
            let generator_faulty: i32 = row.get(0);
            return Ok(generator_faulty);
        }

        Ok(2)
    }

    /// Getting the mains_power_supply value
    /// 0 - there is no power supply from the city power grid
    /// 1 - there is power from the city power grid
    /// 2 - the mains_power_supply value is not 0 or 1.
    pub fn select_mains_power_supply() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query(
            "SELECT mains_power_supply FROM ats_control ORDER BY date DESC limit 1",
            &[],
        )?)
        .into_iter()
        .next()
        {
            let mains_power_supply: i32 = row.get(0);
            return Ok(mains_power_supply);
        }

        Ok(2)
    }

    /// Getting the start_generator value
    /// 0 - generator start failure
    /// 1 - the generator has started
    /// 2 - the start_generator value is not 0 or 1.
    pub fn select_start_generator() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query(
            "SELECT start_generator FROM ats_control ORDER BY date DESC limit 1",
            &[],
        )?)
        .into_iter()
        .next()
        {
            let start_generator: i32 = row.get(0);
            return Ok(start_generator);
        }

        Ok(2)
    }

    /// Getting the transmitted_work value
    /// 0 - mains power is transmitted via ATS
    /// 1- mains power is not transmitted via ATS.
    /// 2 - the transmitted_work value is not 0 or 1.
    pub fn select_transmitted_work() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query(
            "SELECT transmitted_work FROM ats_control ORDER BY date DESC limit 1",
            &[],
        )?)
        .into_iter()
        .next()
        {
            let transmitted_work: i32 = row.get(0);
            return Ok(transmitted_work);
        }

        Ok(2)
    }

    pub fn select_winter_garden() -> Result<WinterGarden, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query("SELECT phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor FROM winter_garden ORDER BY date DESC limit 1", &[])?)
        .into_iter()
        .next()
        {
            let winter_garden = WinterGarden {
                phyto_lighting_1: row.get(0),
                phyto_lighting_2: row.get(1),
                phyto_lighting_3: row.get(2),
                phyto_lighting_4 : row.get(3),
                fan: row.get(4),
                automatic_watering_1: row.get(5),
                automatic_watering_2: row.get(6),
                automatic_watering_3: row.get(7),
                temperature_indoor: row.get(8),
                humidity_indoor: row.get(9),
                illumination_indoor: row.get(10),
                illumination_outdoor: row.get(11),
            };

            return Ok(winter_garden)
        }

        let winter_garden: WinterGarden = WinterGarden::default();
        Ok(winter_garden)
    }

    pub fn select_message_time() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        if let Some(row) = (client.query(
            "SELECT time, date FROM tg_message ORDER BY date DESC limit 1",
            &[],
        )?)
        .into_iter()
        .next()
        {
            let message_time: i32 = row.get(0);
            return Ok(message_time);
        }

        Ok(2)
    }
}
