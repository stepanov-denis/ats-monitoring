pub mod skytable {
    use skytable::actions::Actions;
    use skytable::error;
    use skytable::sync::Connection;

    pub fn set_f64_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new("127.0.0.1", 2003)?;
        con.set(key, value)?;
        Ok(())
    }

    pub fn get_f64_skydb(key: &str) -> Option<String> {
        let mut con = Connection::new("127.0.0.1", 2003).ok()?;
        let x: String = con.get(key).ok()?;
        if x.parse::<f64>().unwrap() > 1.00 {
            Some(x)
        } else {
            None
        }
    }

    pub fn set_i32_skydb(key: &str, value: &str) -> Result<(), error::Error> {
        let mut con = Connection::new("127.0.0.1", 2003)?;
        con.set(key, value)?;
        Ok(())
    }

    pub fn get_i32_skydb(key: &str) -> Option<String> {
        let mut con = Connection::new("127.0.0.1", 2003).ok()?;
        let x: String = con.get(key).ok()?;
        if x.parse::<i32>().unwrap() >= 0 {
            Some(x)
        } else {
            None
        }
    }

    pub fn unix_sql() -> f64 {
        let mut unix_from_sql = crate::skydb::skytable::get_f64_skydb("unix_from_sql");
        let unix_from_sql_str = unix_from_sql.get_or_insert("0".to_string());
        let unix_from_sql_f64 = unix_from_sql_str.parse::<f64>().unwrap();
        unix_from_sql_f64
    }

    pub fn unix_sql_now() -> f64 {
        let mut unix_from_sql_now = crate::skydb::skytable::get_f64_skydb("unix_from_sql_now");
        let unix_from_sql_now_str = unix_from_sql_now.get_or_insert("0".to_string());
        let unix_from_sql_now_f64 = unix_from_sql_now_str.parse::<f64>().unwrap();
        unix_from_sql_now_f64
    }

    pub fn plc_connect() -> i32 {
        let mut plc_connect = crate::skydb::skytable::get_i32_skydb("plc_connect");
        let plc_connect_str = plc_connect.get_or_insert("3".to_string());
        let plc_connect_i32 = plc_connect_str.parse::<i32>().unwrap();
        plc_connect_i32
    }

    pub fn generator_faulty() -> i32 {
        let mut generator_faulty = crate::skydb::skytable::get_i32_skydb("generator_faulty");
        let generator_faulty_str = generator_faulty.get_or_insert("3".to_string());
        let generator_faulty_i32 = generator_faulty_str.parse::<i32>().unwrap();
        generator_faulty_i32
    }
}
