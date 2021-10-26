extern crate chrono;
extern crate timer;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
mod generator_monitoring;
mod modbus_ats;
mod modbus_winter_garden;
mod power_supply_monitoring;

/// Timer for "modbus_ats_spawn" and "modbus_winter_garden_spawn".
fn timer_1sec() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(1), move || {
        tx.send(()).unwrap();
        let _ignored = tx.send(());
    });

    rx.recv().unwrap();
}

/// Timer for "generator_monitoring_spawn" and "power_supply_monitoring_spawn".
fn timer_3sec() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        tx.send(()).unwrap();
        let _ignored = tx.send(());
    });

    rx.recv().unwrap();
}

/// Application workflows.
fn main() {
    let _modbus_ats_spawn = thread::spawn(|| loop {
        modbus_ats::avr_control::avr_control_insert();
        thread::sleep(Duration::from_millis(1));
        timer_1sec();
    });

    let _modbus_winter_garden_spawn = thread::spawn(|| loop {
        modbus_winter_garden::winter_garden::winter_garden_insert();
        thread::sleep(Duration::from_millis(1));
        timer_1sec();
    });

    let _generator_monitoring_spawn = thread::spawn(|| loop {
        generator_monitoring::generator::generator_state();
        thread::sleep(Duration::from_millis(1));
        timer_3sec();
    });

    loop {
        power_supply_monitoring::power_supply::ats_state();
        thread::sleep(Duration::from_millis(1));
        timer_3sec();
    }
}
