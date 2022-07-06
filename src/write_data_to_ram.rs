pub mod write {
    pub fn from_psql_to_skyd() {
        if crate::ram::db::write_to_ram_unix_from_sql().is_ok() {
            info!("write_to_ram_unix_from_sql(): ok");
        } else {
            info!("write_to_ram_unix_from_sql(): error");
        }
        if crate::ram::db::write_to_ram_unix_from_sql_now().is_ok() {
            info!("write_to_ram_unix_from_sql_now(): ok");
        } else {
            info!("write_to_ram_unix_from_sql_now(): error");
        }
        if crate::ram::db::write_to_ram_plc_connect().is_ok() {
            info!("write_to_ram_plc_connect(): ok");
        } else {
            info!("write_to_ram_plc_connect(): error");
        }
        if crate::ram::db::write_to_ram_generator_faulty().is_ok() {
            info!("write_to_ram_generator_faulty(): ok");
        } else {
            info!("write_to_ram_generator_faulty(): error");
        }
        if crate::ram::db::write_to_ram_winter_garden_data_sql().is_ok() {
            info!("write_to_ram_winter_garden_data_sql(): ok");
        } else {
            info!("write_to_ram_winter_garden_data_sql(): error");
        }
    }
}
