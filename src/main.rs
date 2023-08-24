mod temperature;
pub mod ui {
    slint::include_modules!();
}
use ui::*;

fn main() {
    let window = Dashboard::new().unwrap();
    let temperature_join = temperature::setup(&window);
    window.run().unwrap();
    temperature_join.join().unwrap()
}