#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate timer;
use env_logger::{Builder, Target};
use std::io::Error;
use std::thread;
use std::time::Duration;
mod alerts;
mod generator_monitoring;
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;
mod ram;
mod skydb;
mod telegram;

/// Application workflows.
fn main() -> Result<(), Error> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("starting up ats-monitoring app");
    info!("please wait for connecting to plc");
    psql::postgresql::set_transaction_isolation();
    psql::postgresql::create_avr_control_insert_table();
    psql::postgresql::create_log_of_work_app_table();
    psql::postgresql::create_winter_garden_table();
    psql::postgresql::create_generator_load_table();
    psql::postgresql::create_avr_events_table();
    skydb::skytable::set_skyd();

    let _write_data_to_ram_spawn = thread::spawn(|| loop {
        info!("starting up write_data_to_ram_spawn");
        crate::ram::db::write_to_ram_unix_from_sql();
        crate::ram::db::write_to_ram_unix_from_sql_now();
        crate::ram::db::write_to_ram_plc_connect();
        crate::ram::db::write_to_ram_generator_faulty();
        crate::ram::db::write_to_ram_winter_garden_data_sql();
        thread::sleep(Duration::from_millis(3000));
    });

    let _ats_monitoring_bot = thread::spawn(|| loop {
        info!("starting up ats_monitoring_bot");
        telegram::bot::bot_commands();
        thread::sleep(Duration::from_millis(1))
    });

    let _modbus_ats_spawn = thread::spawn(|| loop {
        info!("starting up modbus_ats_spawn");
        modbus_ats::avr_control::avr_control_insert();
        thread::sleep(Duration::from_millis(3000));
    });

    let _modbus_winter_garden_spawn = thread::spawn(|| loop {
        info!("starting up modbus_winter_garden_spawn");
        modbus_winter_garden::winter_garden::winter_garden_insert();
        thread::sleep(Duration::from_millis(3000));
    });

    let _generator_monitoring_spawn = thread::spawn(|| loop {
        info!("starting up generator_monitoring_spawn");
        generator_monitoring::generator::generator_state();
        thread::sleep(Duration::from_millis(3000));
    });

    loop {
        info!("starting up power_supply_monitoring_spawn");
        power_supply_monitoring::power_supply::ats_state();
        thread::sleep(Duration::from_millis(3000));
    }
    Ok(())
}
