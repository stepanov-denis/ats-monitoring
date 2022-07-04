pub mod postgresql {
    use std::error::Error;

    pub fn init_postgres() -> Result<(), Box<dyn Error + Send + Sync>> {
        match crate::psql::postgresql::set_transaction_isolation() {
            Ok(_) => info!("set_transaction_isolation(): ok"),
            Err(e) => info!("{}", e),
        }

        match crate::psql::postgresql::create_avr_control_insert_table() {
            Ok(_) => info!("create_avr_control_insert_table(): ok"),
            Err(e) => info!("{}", e),
        }

        match crate::psql::postgresql::create_log_of_work_app_table() {
            Ok(_) => info!("create_log_of_work_app_table(): ok"),
            Err(e) => info!("{}", e),
        }

        match crate::psql::postgresql::create_winter_garden_table() {
            Ok(_) => info!("create_winter_garden_table(): ok"),
            Err(e) => info!("{}", e),
        }

        match crate::psql::postgresql::create_generator_load_table() {
            Ok(_) => info!("create_generator_load_table(): ok"),
            Err(e) => info!("{}", e),
        }

        match crate::psql::postgresql::create_avr_events_table() {
            Ok(_) => info!("create_avr_events_table(): ok"),
            Err(e) => info!("{}", e),
        }

        Ok(())
    }
}

pub mod skyd {
    use std::error::Error;

    pub fn init_skyd() -> Result<(), Box<dyn Error + Send + Sync>> {
        match crate::skydb::skytable::set_f64_skydb("unix_from_sql", &0.00.to_string()) {
            Ok(_) => info!("set unix_from_sql: 0.00"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_f64_skydb("unix_from_sql_now", &0.00.to_string()) {
            Ok(_) => info!("set unix_from_sql_now: 0.00"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("plc_connect", &0.to_string()) {
            Ok(_) => info!("set plc_connect: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("generator_faulty", &0.to_string()) {
            Ok(_) => info!("set generator_faulty: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("mains_power_supply", &0.to_string()) {
            Ok(_) => info!("set mains_power_supply: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("start_generator", &0.to_string()) {
            Ok(_) => info!("set start_generator: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("generator_work", &0.to_string()) {
            Ok(_) => info!("set generator_work: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("phyto_lighting_1", &0.to_string()) {
            Ok(_) => info!("set phyto_lighting_1: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("phyto_lighting_2", &0.to_string()) {
            Ok(_) => info!("set phyto_lighting_2: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("phyto_lighting_3", &0.to_string()) {
            Ok(_) => info!("set phyto_lighting_3: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("phyto_lighting_4", &0.to_string()) {
            Ok(_) => info!("set phyto_lighting_4: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("fan", &0.to_string()) {
            Ok(_) => info!("set fan: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("automatic_watering_1", &0.to_string()) {
            Ok(_) => info!("set automatic_watering_1: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("automatic_watering_2", &0.to_string()) {
            Ok(_) => info!("set automatic_watering_2: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("automatic_watering_3", &0.to_string()) {
            Ok(_) => info!("set automatic_watering_3: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("temperature_indoor", &0.to_string()) {
            Ok(_) => info!("set temperature_indoor: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("humidity_indoor", &0.to_string()) {
            Ok(_) => info!("set humidity_indoor: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("illumination_indoor", &0.to_string()) {
            Ok(_) => info!("set illumination_indoor: 0"),
            Err(e) => info!("{}", e),
        }

        match crate::skydb::skytable::set_i32_skydb("illumination_outdoor", &0.to_string()) {
            Ok(_) => info!("set illumination_outdoor: 0"),
            Err(e) => info!("{}", e),
        }

        Ok(())
    }
}
