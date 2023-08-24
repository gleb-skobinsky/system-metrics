use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

use rand::Rng;
use slint;
use slint::ComponentHandle;

use crate::ui;

fn generate_svg(values: &[f32]) -> String {
    let mut svg = String::new();
    let mut x = 0;

    for (i, value) in values.iter().enumerate() {
        let mut mark = "L";
        if i == 0 {
            mark = "M"
        }
        svg.push_str(&format!("{} {} {} ", mark, x, -value));
        x += 10;
    }
    svg
}

fn update_vector(vector: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();
    let new_value = rng.gen_range(10.0..=100.0);

    vector.remove(0);
    vector.push(new_value);
}

pub fn setup<T: Send + 'static>(window: &ui::Dashboard, receiver: Receiver<T>) -> JoinHandle<()> {
    let window_weak = window.as_weak();

    thread::spawn(move ||
        worker_loop(window_weak, receiver)
    )
}

fn worker_loop<T>(window_weak: slint::Weak<ui::Dashboard>, receiver: Receiver<T>) {
    let mut vector: Vec<f32> = Vec::with_capacity(20);
    let mut path: String = "".to_string();

    for _ in 0..20 {
        let value: f32 = rand::thread_rng().gen_range(10.0..=100.0);
        vector.push(value);
        path = generate_svg(&vector);
    }
    display_current(window_weak.clone(), path);

    loop {
        match receiver.try_recv() {
            Ok(_) => { break; }
            Err(_) => {}
        }
        update_vector(&mut vector);
        path = generate_svg(&vector);
        display_current(window_weak.clone(), path);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn display_current(window_weak: slint::Weak<ui::Dashboard>, path: String) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            window
                .global::<ui::TemperatureAdapter>()
                .set_path(slint::SharedString::from(path));
        })
        .unwrap();
}
