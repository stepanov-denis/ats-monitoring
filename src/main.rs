#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate timer;
use env_logger::{Builder, Target};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod alerts;
mod generator_monitoring;
mod init;
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;
mod ram;
mod read_env;
mod skydb;
mod telegram;
mod write_data_to_ram;

/// Application workflows.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("starting up ats-monitoring app");
    info!("please wait...");
    // println!("{}", crate::psql::postgresql::db_connect());
    info!("init postresql");
    init::postgresql::init_postgres();

    info!("init skyd");
    init::skyd::init_skyd();

    let _write_data_to_ram_spawn = thread::spawn(|| loop {
        info!("starting up write_data_to_ram_spawn");
        write_data_to_ram::write::from_psql_to_skyd();
        thread::sleep(Duration::from_millis(1000));
    });

    let _modbus_ats_spawn = thread::spawn(|| loop {
        info!("starting up modbus_ats_spawn");
        modbus_ats::avr_control::avr_control();
        thread::sleep(Duration::from_millis(1000));
    });

    let _modbus_winter_garden_spawn = thread::spawn(|| loop {
        info!("starting up modbus_winter_garden_spawn");
        if modbus_winter_garden::winter_garden::winter_garden_insert().is_ok() {
            info!("winter_garden_insert(): ok");
        } else {
            info!("winter_garden_insert(): error");
        }
        thread::sleep(Duration::from_millis(1000));
    });

    let _generator_monitoring_spawn = thread::spawn(|| loop {
        info!("starting up generator_monitoring_spawn");
        if generator_monitoring::generator::generator_state().is_ok() {
            info!("generator_state(): ok");
        } else {
            info!("generator_state(): error");
        }
        thread::sleep(Duration::from_millis(1000));
    });

    let _ats_monitoring_bot = thread::spawn(|| loop {
        info!("starting up ats_monitoring_bot");
        telegram::bot::bot_commands();
        thread::sleep(Duration::from_millis(1))
    });

    loop {
        info!("starting up power_supply_monitoring_spawn");
        if power_supply_monitoring::power_supply::ats_state().is_ok() {
            info!("ats_state(): ok");
        } else {
            info!("ats_state(): error");
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
