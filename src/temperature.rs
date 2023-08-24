use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use slint;
use slint::ComponentHandle;
use sysinfo::{CpuExt, System, SystemExt};

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

fn update_vector(vector: &mut Vec<f32>, sys: &mut System) {
    let usage = get_cpu_usage(sys);

    vector.remove(0);
    vector.push(usage);
}

pub fn setup<T: Send + 'static>(window: &ui::Dashboard, receiver: Receiver<T>) -> JoinHandle<()> {
    let window_weak = window.as_weak();


    thread::spawn(move ||
        worker_loop(window_weak, receiver)
    )
}

fn worker_loop<T>(window_weak: slint::Weak<ui::Dashboard>, receiver: Receiver<T>) {
    let mut vector: Vec<f32> = Vec::with_capacity(20);
    let mut sys = System::new();
    let usage = get_cpu_usage(&mut sys);

    for _ in 0..20 {
        vector.push(usage.clone());
    }
    let mut path  = generate_svg(&vector);
    display_current(window_weak.clone(), path);

    loop {
        match receiver.try_recv() {
            Ok(_) => { break; }
            Err(_) => {}
        }
        update_vector(&mut vector, &mut sys);
        path = generate_svg(&vector);
        display_current(window_weak.clone(), path);
        thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}

fn get_cpu_usage(sys: &mut System) -> f32 {
    sys.refresh_cpu();
    let mut usage: Vec<f32> = Vec::new();
    for cpu in sys.cpus() {
        usage.push(cpu.cpu_usage());
    }
    return average(usage);
}

fn average(numbers: Vec<f32>) -> f32 {
    // Remember how many numbers we were passed.
    let nnumbers = numbers.len() as f32;
    let mut sum = 0.0;
    // This will consume the numbers.
    for n in numbers {
        sum += n;
    }
    // Average (arithmetic mean) is sum divided by count.
    sum / nnumbers
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
