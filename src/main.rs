#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate modbus_iiot;
extern crate timer;
use env_logger::{Builder, Target};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod generator_monitoring;
mod init;
mod json;
mod logger;
mod modbus_ats;
mod modbus_client;
mod modbus_winter_garden;
mod power_supply_monitoring;
mod psql;
mod read_env;
mod sms;
mod tg;

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
        info!("starting up modbus_ats_thread");
        modbus_ats::ats_control::ats();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run polling of the automatic winter garden management system.
    let _modbus_winter_garden_thread = thread::spawn(|| loop {
        info!("starting up modbus_winter_garden_thread");
        crate::modbus_winter_garden::winter_garden_control::winter_garden();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run the monitoring of the generator.
    let _generator_monitoring_thread = thread::spawn(|| loop {
        info!("starting up generator_monitoring_thread");
        generator_monitoring::generator::generator_monitoring();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run Telegram-bot.
    let _ats_monitoring_bot_thread = thread::spawn(|| loop {
        info!("starting up ats_monitoring_bot_thread");
        crate::tg::api::callback_winter_garden();
        // telegram::bot::bot_commands();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run the monitoring of the automatic reserve input.
    loop {
        info!("starting up power_supply_monitoring_thread");
        power_supply_monitoring::power_supply::ats_monitoring();
        thread::sleep(Duration::from_millis(1000));
    }
}
