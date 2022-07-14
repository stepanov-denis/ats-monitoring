#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate timer;
use env_logger::{Builder, Target};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod generator_monitoring;
mod init;
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;
mod read_env;
mod telegram;
mod sms;

/// Application workflows.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initializing a Logger.
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    info!("starting up ats-monitoring app");
    info!("init postresql");

    // Set the transaction isolation level and
    // create tables in Postgresql.
    init::postgresql::init_postgres();

    // Run polling of the automatic reserve input.
    let _modbus_ats_thread = thread::spawn(|| loop {
        info!("starting up modbus_ats_spawn");
        modbus_ats::avr_control::avr_control();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run polling of the automatic winter garden management system.
    let _modbus_winter_garden_thread = thread::spawn(|| loop {
        info!("starting up modbus_winter_garden_spawn");
        match modbus_winter_garden::winter_garden::winter_garden() {
            Ok(_) => info!("winter_garden_insert(): ok"),
            Err(e) => info!("{}", e),
        }
        thread::sleep(Duration::from_millis(1000));
    });

    // Run the monitoring of the generator.
    let _generator_monitoring_thread = thread::spawn(|| loop {
        info!("starting up generator_monitoring_spawn");
        generator_monitoring::generator::generator_state();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run Telegram-bot.
    let _ats_monitoring_bot = thread::spawn(|| loop {
        info!("starting up ats_monitoring_bot");
        telegram::bot::bot_commands();
        thread::sleep(Duration::from_millis(1))
    });

    // Run the monitoring of the automatic reserve input.
    loop {
        info!("starting up power_supply_monitoring_spawn");
        power_supply_monitoring::power_supply::ats_state();
        thread::sleep(Duration::from_millis(1000));
    }
}
