extern crate chrono;
extern crate timer;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
mod generator_monitoring;
mod power_supply_monitoring;

/// Timer for adjusting the execution time of thread loops.
fn timer_3sec() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        tx.send(()).unwrap();
        let _ignored = tx.send(());
    });

    rx.recv().unwrap();
}

fn main() {
    let _handle = thread::spawn(|| loop {
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
