mod updater;
mod math;
mod svg;
mod cpu;
mod temperature;
mod ram;

pub mod ui {
    slint::include_modules!();
}
use ui::*;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<Result<String, &str>>();
    let window = Dashboard::new().unwrap();
    let temperature_join = updater::setup(&window, receiver);
    window.run().unwrap();
    let _ = sender.send(Ok("Terminate monitoring loop".to_owned()));
    temperature_join.join().unwrap()
}