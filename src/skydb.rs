pub mod skytable {
    use skytable::actions::Actions;
    use skytable::error;
    use skytable::sync::Connection;

    pub fn skyd_connect() -> String {
        let string_connection = String::from("skytable");
        string_connection
    }

    /// Record f64 to skyd
    pub fn set_f64_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new(&skyd_connect(), 2003)?;
        con.set(key, value)?;
        Ok(())
    }

    /// Update f64 to skyd
    pub fn update_f64_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new(&skyd_connect(), 2003)?;
        con.update(key, value)?;
        Ok(())
    }

    /// Write f64 from skyd
    pub fn get_f64_skydb(key: &str) -> Option<String> {
        let mut con = Connection::new(&skyd_connect(), 2003).ok()?;
        let x: String = con.get(key).ok()?;
        if x.parse::<f64>().is_ok() {
            Some(x)
        } else {
            None
        }
    }

    /// Record i32 to skyd
    pub fn set_i32_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new(&skyd_connect(), 2003)?;
        con.set(key, value)?;
        Ok(())
    }

    /// Update i32 to skyd
    pub fn update_i32_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new(&skyd_connect(), 2003)?;
        con.update(key, value)?;
        Ok(())
    }

    /// Write i32 from skyd
    pub fn get_i32_skydb(key: &str) -> Option<String> {
        let mut con = Connection::new(&skyd_connect(), 2003).ok()?;
        let x: String = con.get(key).ok()?;
        if x.parse::<i32>().is_ok() {
            Some(x)
        } else {
            None
        }
    }

    /// Value unix time of the lastest record in table avr_control_insert
    pub fn unix_sql() -> f64 {
        let mut unix_from_sql = get_f64_skydb("unix_from_sql");
        let unix_from_sql_str = unix_from_sql.get_or_insert("0".to_string());
        unix_from_sql_str.parse::<f64>().unwrap()
    }

    /// Value of unix time now
    pub fn unix_sql_now() -> f64 {
        let mut unix_from_sql_now = get_f64_skydb("unix_from_sql_now");
        let unix_from_sql_now_str = unix_from_sql_now.get_or_insert("1000".to_string());
        unix_from_sql_now_str.parse::<f64>().unwrap()
    }

    /// PLC connection status
    pub fn plc_connect() -> i32 {
        let mut plc_connect = get_i32_skydb("plc_connect");
        let plc_connect_str = plc_connect.get_or_insert("2".to_string());
        plc_connect_str.parse::<i32>().unwrap()
    }

    /// Generator fault status
    pub fn generator_faulty() -> i32 {
        let mut generator_faulty = get_i32_skydb("generator_faulty");
        let generator_faulty_str = generator_faulty.get_or_insert("2".to_string());
        generator_faulty_str.parse::<i32>().unwrap()
    }

    /// Mains power status
    pub fn mains_power_supply() -> i32 {
        let mut mains_power_supply = get_i32_skydb("mains_power_supply");
        let mains_power_supply_str = mains_power_supply.get_or_insert("2".to_string());
        mains_power_supply_str.parse::<i32>().unwrap()
    }

    /// Start generator status
    pub fn start_generator() -> i32 {
        let mut start_generator = get_i32_skydb("start_generator");
        let start_generator_str = start_generator.get_or_insert("2".to_string());
        start_generator_str.parse::<i32>().unwrap()
    }

    /// Generator work status
    pub fn generator_work() -> i32 {
        let mut generator_work = get_i32_skydb("generator_work");
        let generator_work_str = generator_work.get_or_insert("2".to_string());
        generator_work_str.parse::<i32>().unwrap()
    }

    pub fn set_skyd() {
        set_f64_skydb("unix_from_sql", &0.00.to_string());
        set_f64_skydb("unix_from_sql_now", &0.00.to_string());
        set_i32_skydb("plc_connect", &0.to_string());
        set_i32_skydb("generator_faulty", &0.to_string());
        set_i32_skydb("mains_power_supply", &0.to_string());
        set_i32_skydb("start_generator", &0.to_string());
        set_i32_skydb("generator_work", &0.to_string());
        set_i32_skydb("phyto_lighting_1", &0.to_string());
        set_i32_skydb("phyto_lighting_2", &0.to_string());
        set_i32_skydb("phyto_lighting_3", &0.to_string());
        set_i32_skydb("phyto_lighting_4", &0.to_string());
        set_i32_skydb("fan", &0.to_string());
        set_i32_skydb("automatic_watering_1", &0.to_string());
        set_i32_skydb("automatic_watering_2", &0.to_string());
        set_i32_skydb("automatic_watering_3", &0.to_string());
        set_i32_skydb("temperature_indoor", &0.to_string());
        set_i32_skydb("humidity_indoor", &0.to_string());
        set_i32_skydb("illumination_indoor", &0.to_string());
        set_i32_skydb("illumination_outdoor", &0.to_string());
    }
}
