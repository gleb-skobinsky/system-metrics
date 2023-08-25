use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

use slint;
use slint::{ComponentHandle, SharedString, VecModel};
use sysinfo::{CpuExt, System, SystemExt};

use crate::math::average;
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

pub fn setup<T: Send + 'static>(window: &ui::Dashboard, receiver: Receiver<T>) -> JoinHandle<()> {
    let window_weak = window.as_weak();
    thread::spawn(move ||
        worker_loop(window_weak, receiver)
    )
}

fn worker_loop<T>(window_weak: slint::Weak<ui::Dashboard>, receiver: Receiver<T>) {
    let mut sys = System::new();
    let mut chart = Vec::default();
    sys.refresh_cpu();
    for cpu in sys.cpus() {
        let mut vector: Vec<f32> = Vec::with_capacity(20);
        let usage = cpu.cpu_usage();
        for _ in 0..20 {
            vector.push(usage.clone());
        }
        chart.push(vector);
    }

    loop {
        match receiver.try_recv() {
            Ok(_) => { break; }
            Err(_) => {}
        }

        sys.refresh_cpu();
        let cpus = sys.cpus();
        for (i, cpu) in cpus.iter().enumerate() {
            let usage = cpu.cpu_usage();
            chart[i].remove(0);
            chart[i.clone()].push(usage);
        }
        display_current(window_weak.clone(), chart.clone());
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

fn chart_to_chart_model(chart: &Vec<Vec<f32>>) -> VecModel<SharedString> {
    let chart_model = VecModel::default();
    for line in chart {
        chart_model.push(SharedString::from(generate_svg(&line)))
    }
    return chart_model;
}


fn display_current(window_weak: slint::Weak<ui::Dashboard>, chart: Vec<Vec<f32>>) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            window
                .global::<ui::MainViewModel>()
                .set_cpu_data(Rc::new(chart_to_chart_model(&chart)).into());
        })
        .unwrap();
}