use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use slint;
use slint::{ComponentHandle, SharedString, VecModel};
use sysinfo::{System, SystemExt};
use crate::cpu::{init_cpu, update_cpu};

use crate::ui;
use crate::svg::generate_svg;
use crate::temperature::{init_temp, update_temp};

pub fn setup<T: Send + 'static>(window: &ui::Dashboard, receiver: Receiver<T>) -> JoinHandle<()> {
    let window_weak = window.as_weak();
    thread::spawn(move ||
        worker_loop(window_weak, receiver)
    )
}

fn worker_loop<T>(window_weak: slint::Weak<ui::Dashboard>, receiver: Receiver<T>) {
    let mut sys = System::new();
    let mut cpu_chart: Vec<Vec<f32>> = Vec::default();
    let mut temp_chart: Vec<Vec<f32>> = Vec::default();
    init_cpu(&mut sys, &mut cpu_chart);
    init_temp(&mut sys, &mut temp_chart);
    loop {
        match receiver.try_recv() {
            Ok(_) => { break; }
            Err(_) => {}
        }
        update_cpu(&mut sys, &mut cpu_chart);
        update_temp(&mut sys, &mut temp_chart);
        display_current(&window_weak, cpu_chart.clone(), temp_chart.clone());
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


fn display_current(
    window_weak: &slint::Weak<ui::Dashboard>,
    cpu_chart: Vec<Vec<f32>>,
    temp_chart: Vec<Vec<f32>>
) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            let cpu_model = chart_to_chart_model(&cpu_chart);
            let temp_model = chart_to_chart_model(&temp_chart);
            let vm = window.global::<ui::MainViewModel>();
            vm.set_cpu_data(Rc::new(cpu_model).into());
            vm.set_temperature_data(Rc::new(temp_model).into())
        })
        .unwrap();
}