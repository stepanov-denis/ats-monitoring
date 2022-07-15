pub mod postgresql {
    use postgres::{Client, Error as PostgresError, NoTls};

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
                    mains_power_supply int NOT NULL,
                    start_generator int NOT NULL,
                    generator_faulty int NOT NULL,
                    transmitted_work int NOT NULL,
                    connection int NOT NULL,
                    mark timestamptz default current_timestamp
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
                    event text NOT NULL,
                    mark timestamp default current_timestamp
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
                    phyto_lighting_1 int NOT NULL,
                    phyto_lighting_2 int NOT NULL,
                    phyto_lighting_3 int NOT NULL,
                    phyto_lighting_4 int NOT NULL,
                    fan int NOT NULL,
                    automatic_watering_1 int NOT NULL,
                    automatic_watering_2 int NOT NULL,
                    automatic_watering_3 int NOT NULL,
                    temperature_indoor int NOT NULL,
                    humidity_indoor int NOT NULL,
                    illumination_indoor int NOT NULL,
                    illumination_outdoor int NOT NULL,
                    mark timestamp default current_timestamp
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
                    load int NOT NULL,
                    mark timestamp default current_timestamp
                
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
            "SELECT event, mark FROM app_log ORDER BY mark DESC limit 1",
            &[],
        )? {
            let event: &str = row.get(0);

            info!("entry in the sql table 'app_log': {}", event);
        }
        Ok(())
    }

    /// Records the values of the variables of the automatic reserve to the SQL table "ats_control".
    pub fn insert_ats(
        mains_power_supply: i32,
        start_generator: i32,
        generator_faulty: i32,
        transmitted_work: i32,
        connection: i32,
    ) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute(
            "INSERT INTO ats_control (mains_power_supply, start_generator, generator_faulty, transmitted_work, connection) VALUES ($1, $2, $3, $4, $5)",
            &[&mains_power_supply, &start_generator, &generator_faulty, &transmitted_work, &connection],
        )?;

        for row in client.query("SELECT mains_power_supply, start_generator, generator_faulty, transmitted_work, connection FROM ats_control ORDER BY mark DESC limit 1", &[])? {
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

    /// Records the values of the variables of the automatic winter garden management system to the SQL table "winter_garden".
    pub fn insert_winter_garden(
        phyto_lighting_1: i32,
        phyto_lighting_2: i32,
        phyto_lighting_3: i32,
        phyto_lighting_4: i32,
        fan: i32,
        automatic_watering_1: i32,
        automatic_watering_2: i32,
        automatic_watering_3: i32,
        temperature_indoor: i32,
        humidity_indoor: i32,
        illumination_indoor: i32,
        illumination_outdoor: i32,
    ) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute(
            "INSERT INTO winter_garden (phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[&phyto_lighting_1, &phyto_lighting_2, &phyto_lighting_3, &phyto_lighting_4, &fan, &automatic_watering_1, &automatic_watering_2, &automatic_watering_3, &temperature_indoor, &humidity_indoor, &illumination_indoor, &illumination_outdoor],
        )?;

        for row in client.query("SELECT phyto_lighting_1, phyto_lighting_2, phyto_lighting_3, phyto_lighting_4, fan, automatic_watering_1, automatic_watering_2, automatic_watering_3, temperature_indoor, humidity_indoor, illumination_indoor, illumination_outdoor FROM winter_garden ORDER BY mark DESC limit 1", &[])? {
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

    pub fn insert_generator_load(load: i32) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        client.execute("INSERT INTO generator_load (load) VALUES ($1)", &[&load])?;

        for row in client.query(
            "SELECT load FROM generator_load ORDER BY mark DESC limit 1",
            &[],
        )? {
            let load: i32 = row.get(0);
            info!(
                "the following values are read from the plc and written to the table 'generator_load' load: {}",
                load);
        }
        Ok(())
    }

    pub fn select_generator_faulty() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        for row in client.query(
            "SELECT generator_faulty FROM ats_control ORDER BY mark DESC limit 1",
            &[],
        )? {
            let generator_faulty: i32 = row.get(0);
            return Ok(generator_faulty);
        }
        Ok(2)
    }

    /// Getting the start_generator value
    /// 0 - generator startup failure
    /// 1 - successful generator startup
    pub fn select_mains_power_supply() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        for row in client.query(
            "SELECT mains_power_supply FROM ats_control ORDER BY mark DESC limit 1",
            &[],
        )? {
            let mains_power_supply: i32 = row.get(0);
            return Ok(mains_power_supply);
        }
        Ok(2)
    }

    pub fn select_start_generator() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        for row in client.query(
            "SELECT start_generator FROM ats_control ORDER BY mark DESC limit 1",
            &[],
        )? {
            let start_generator: i32 = row.get(0);
            return Ok(start_generator);
        }
        Ok(2)
    }

    /// Getting the generator_work value
    /// 0 - mains power is transmitted via ATS
    /// 1- mains power is not transmitted via ATS.
    pub fn select_transmitted_work() -> Result<i32, PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;

        for row in client.query(
            "SELECT transmitted_work FROM ats_control ORDER BY mark DESC limit 1",
            &[],
        )? {
            let transmitted_work: i32 = row.get(0);
            return Ok(transmitted_work);
        }
        Ok(2)
    }
}
