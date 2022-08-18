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
    let _ats = thread::spawn(|| loop {
        info!("starting up ats thread");
        modbus_ats::ats_control::ats();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run polling of the automatic winter garden management system.
    let _winter_garden = thread::spawn(|| loop {
        info!("starting up winter_garden thread");
        crate::modbus_winter_garden::winter_garden_control::winter_garden();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run the monitoring of the generator.
    let _generator_monitoring = thread::spawn(|| loop {
        info!("starting up generator_monitoring thread");
        generator_monitoring::generator::generator_monitoring();
        thread::sleep(Duration::from_millis(1000));
    });

    // Run Telegram-bot.
    let _callback = thread::spawn(|| loop {
        info!("starting up callback thread");
        crate::tg::api::callback();
        thread::sleep(Duration::from_millis(1));
    });

    let _update_chat_id = thread::spawn(|| loop {
        info!("starting up update_chat_id thread");
        crate::tg::api::update_chat_id();
        thread::sleep(Duration::from_millis(1));
    });

    // Run the monitoring of the automatic reserve input.
    loop {
        info!("starting up ats_monitoring thread");
        power_supply_monitoring::power_supply::ats_monitoring();
        thread::sleep(Duration::from_millis(1000));
    }
}
