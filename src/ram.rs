pub mod db {
    #[macro_use]
    use postgres::{Client, Error as PostgresError, NoTls};

    /// The structure of the generator failure.
    pub struct Faulty {
        generator_faulty: i32,
    }

    /// The structure of a UNIX timestamp with the time zone of the last value entry in the table.
    pub struct UnixFromSql {
        time: f64,
    }

    /// The structure of a UNIX timestamp with the time zone now.
    pub struct UnixFromSqlNow {
        time: f64,
    }

    /// The structure of the signal of the presence of the opc server connection with the plc.
    pub struct PlcConnect {
        connection: i32,
    }

    /// The structure of power supply from the power grid.
    pub struct PowerSupply {
        mains_power_supply: i32,
        start_generator: i32,
        generator_work: i32,
    }

    /// Get the time (unix) of the last entry in a table "avr_control_insert" and write it to the db "skydb" in RAM
    pub fn write_to_ram_unix_from_sql() -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        for row in client.query(
            "SELECT EXTRACT(epoch FROM mark) FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let unix_from_sql = UnixFromSql { time: row.get(0) };
            crate::skydb::skytable::set_f64_skydb("unix_from_sql", &unix_from_sql.time.to_string());
            info!(
                "Time (unix) of the last entry in a table 'avr_control_insert': Postgres = {}, Skytable = {}",
                unix_from_sql.time, crate::skydb::skytable::unix_sql()
            );
        }
        Ok(())
    }

    /// Get the time (unix) now from PostgreSQL and write it to the db "skydb" in RAM
    pub fn write_to_ram_unix_from_sql_now() -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        for row in client.query(
            "SELECT EXTRACT(epoch FROM now()) FROM avr_control_insert ORDER BY now() DESC limit 1",
            &[],
        )? {
            let unix_from_sql_now = UnixFromSqlNow { time: row.get(0) };
            crate::skydb::skytable::set_f64_skydb(
                "unix_from_sql_now",
                &unix_from_sql_now.time.to_string(),
            );
            info!(
                "Time (unix) now from PostgreSQL = {}",
                unix_from_sql_now.time
            );
        }
        Ok(())
    }

    /// Get latest value of plc_connect from PostgreSQL and write to the db "skydb" in RAM
    pub fn write_to_ram_plc_connect() -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        for row in client.query(
            "SELECT mark, connection FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let plc_connect = PlcConnect {
                connection: row.get(1),
            };
            crate::skydb::skytable::set_i32_skydb(
                "plc_connect",
                &plc_connect.connection.to_string(),
            );
            info!(
                "Latest value of plc_connect from PostgreSQL = {}",
                plc_connect.connection
            );
        }
        Ok(())
    }

    /// Get latest value of generator_faulty from PostgreSQL and write to the db "skydb" in RAM
    pub fn write_to_ram_generator_faulty() -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        for row in client.query(
            "SELECT mark, generator_faulty FROM avr_control_insert ORDER BY mark DESC limit 1",
            &[],
        )? {
            let faulty = &Faulty {
                generator_faulty: row.get(1),
            };
            crate::skydb::skytable::set_i32_skydb(
                "generator_faulty",
                &faulty.generator_faulty.to_string(),
            );
            info!(
                "Latest value of generator_faulty from PostgreSQL = {}",
                faulty.generator_faulty
            );
        }
        Ok(())
    }

    /// Get latest values of main_power_supply, start_generator, generator_work from PostgreSQL and write to the db "skydb" in RAM
    pub fn write_to_ram_mains_power_supply_start_generator_generator_work(
    ) -> Result<(), PostgresError> {
        let mut client = Client::connect(&crate::psql::postgresql::db_connect(), NoTls)?;
        for row in client
                .query("SELECT mains_power_supply, start_generator, generator_work, mark FROM avr_control_insert ORDER BY mark DESC limit 1", &[])
                ?
            {
                let powersupply = PowerSupply {
                    mains_power_supply: row.get(0),
                    start_generator: row.get(1),
                    generator_work: row.get(2),
                };
                crate::skydb::skytable::set_i32_skydb(
                    "mains_power_supply",
                    &powersupply.mains_power_supply.to_string(),
                );
                info!(
                    "Latest value of mains_power_supply from PostgreSQL = {}",
                    powersupply.mains_power_supply
                );
                crate::skydb::skytable::set_i32_skydb(
                    "start_generator",
                    &powersupply.start_generator.to_string(),
                );
                info!(
                    "Latest value of start_generator from PostgreSQL = {}",
                    powersupply.start_generator
                );
                crate::skydb::skytable::set_i32_skydb(
                    "generator_work",
                    &powersupply.generator_work.to_string(),
                );
                info!(
                    "Latest value of generator_work from PostgreSQL = {}",
                    powersupply.generator_work
                );
            }
        Ok(())
    }
}
