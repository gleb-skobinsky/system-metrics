use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use slint;
use slint::{ComponentHandle, SharedString, VecModel};
use sysinfo::{CpuExt, System, SystemExt};

use crate::ui;
use crate::svg::generate_svg;

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
            vector.push(usage);
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
            chart[*&i].remove(0);
            chart[*&i].push(usage);
        }
        display_current(&window_weak, chart.clone());
        thread::sleep(Duration::from_secs(1));
    }
}

fn chart_to_chart_model(chart: &Vec<Vec<f32>>) -> VecModel<SharedString> {
    let chart_model = VecModel::default();
    for line in chart {
        chart_model.push(SharedString::from(generate_svg(&line)))
    }
    return chart_model;
}


fn display_current(window_weak: &slint::Weak<ui::Dashboard>, chart: Vec<Vec<f32>>) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            let model = chart_to_chart_model(&chart);
            let vm = window.global::<ui::MainViewModel>();
            vm.set_cpu_data(Rc::new(model).into());
            vm.invoke_refresh_status();
        })
        .unwrap();
}