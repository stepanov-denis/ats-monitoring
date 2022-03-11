extern crate chrono;
extern crate timer;
use std::thread;
use std::time::Duration;
mod generator_monitoring;
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;

/// Application workflows.
fn main() {
    psql::postgresql::set_transaction_isolation();
    psql::postgresql::create_avr_control_insert_table();
    psql::postgresql::create_log_of_work_app_table();
    psql::postgresql::create_winter_garden_table();
    psql::postgresql::create_generator_load_table();
    psql::postgresql::create_avr_events_table();
    println!("Starting ATS Monitoring app");
    println!("Please wait for connecting to PLC");
    let _modbus_ats_spawn = thread::spawn(|| loop {
        modbus_ats::avr_control::avr_control_insert();
        thread::sleep(Duration::from_millis(3000));
    });

    let _modbus_winter_garden_spawn = thread::spawn(|| loop {
        modbus_winter_garden::winter_garden::winter_garden_insert();
        thread::sleep(Duration::from_millis(3000));
    });

    let _generator_monitoring_spawn = thread::spawn(|| loop {
        generator_monitoring::generator::generator_state();
        thread::sleep(Duration::from_millis(3000));
    });

    loop {
        power_supply_monitoring::power_supply::ats_state();
        thread::sleep(Duration::from_millis(3000));
    }
}
