pub mod postgresql {

    pub fn init_postgres() {
        crate::psql::postgresql::set_transaction_isolation().unwrap();

        crate::psql::postgresql::create_avr_control_insert_table().unwrap();

        crate::psql::postgresql::create_log_of_work_app_table().unwrap();

        crate::psql::postgresql::create_winter_garden_table().unwrap();

        crate::psql::postgresql::create_generator_load_table().unwrap();

        crate::psql::postgresql::create_avr_events_table().unwrap();
    }
}

pub mod skyd {

    pub fn init_skyd() {
        crate::skydb::skytable::set_f64_skydb("unix_from_sql", &0.00.to_string()).unwrap();

        crate::skydb::skytable::set_f64_skydb("unix_from_sql_now", &0.00.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("plc_connect", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("generator_faulty", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("mains_power_supply", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("start_generator", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("generator_work", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("phyto_lighting_1", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("phyto_lighting_2", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("phyto_lighting_3", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("phyto_lighting_4", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("fan", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("automatic_watering_1", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("automatic_watering_2", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("automatic_watering_3", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("temperature_indoor", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("humidity_indoor", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("illumination_indoor", &0.to_string()).unwrap();

        crate::skydb::skytable::set_i32_skydb("illumination_outdoor", &0.to_string()).unwrap();
    }
}
