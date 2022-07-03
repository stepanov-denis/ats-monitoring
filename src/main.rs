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
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;
mod ram;
mod skydb;
mod telegram;

/// Application workflows.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("starting up ats-monitoring app");
    info!("please wait...");
    if psql::postgresql::set_transaction_isolation().is_ok() {
        info!("set_transaction_isolation(): ok");
    } else {
        info!("set_transaction_isolation(): error");
    }
    if psql::postgresql::create_avr_control_insert_table().is_ok() {
        info!("create_avr_control_insert_table(): ok");
    } else {
        info!("create_avr_control_insert_table(): error");
    }
    if psql::postgresql::create_log_of_work_app_table().is_ok() {
        info!("create_log_of_work_app_table(): ok");
    } else {
        info!("create_log_of_work_app_table(): error");
    }
    if psql::postgresql::create_winter_garden_table().is_ok() {
        info!("create_winter_garden_table(): ok");
    } else {
        info!("create_winter_garden_table(): ok");
    }
    if psql::postgresql::create_generator_load_table().is_ok() {
        info!("create_generator_load_table(): ok");
    } else {
        info!("create_generator_load_table(): error");
    }
    if psql::postgresql::create_avr_events_table().is_ok() {
        info!("create_avr_events_table(): ok");
    } else {
        info!("create_avr_events_table(): error");
    }
    if skydb::skytable::set_skyd().is_ok() {
        info!("set_skyd(): ok");
    } else {
        info!("set_skyd(): error");
    }

    let _write_data_to_ram_spawn = thread::spawn(|| loop {
        info!("starting up write_data_to_ram_spawn");
        if ram::db::write_to_ram_unix_from_sql().is_ok() {
            info!("write_to_ram_unix_from_sql(): ok");
        } else {
            info!("write_to_ram_unix_from_sql(): error");
        }
        if ram::db::write_to_ram_unix_from_sql_now().is_ok() {
            info!("write_to_ram_unix_from_sql_now(): ok");
        } else {
            info!("write_to_ram_unix_from_sql_now(): error");
        }
        if ram::db::write_to_ram_plc_connect().is_ok() {
            info!("write_to_ram_plc_connect(): ok");
        } else {
            info!("write_to_ram_plc_connect(): error");
        }
        if ram::db::write_to_ram_generator_faulty().is_ok() {
            info!("write_to_ram_generator_faulty(): ok");
        } else {
            info!("write_to_ram_generator_faulty(): error");
        }
        if ram::db::write_to_ram_winter_garden_data_sql().is_ok() {
            info!("write_to_ram_winter_garden_data_sql(): ok");
        } else {
            info!("write_to_ram_winter_garden_data_sql(): error");
        }
        thread::sleep(Duration::from_millis(1000));
    });

    let _modbus_ats_spawn = thread::spawn(|| loop {
        info!("starting up modbus_ats_spawn");
        if modbus_ats::avr_control::avr_control().is_ok() {
            info!("avr_control_insert(): ok");
        } else {
            info!("avr_control_insert(): error");
        }
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
