use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use slint::{self, ModelRc};
use slint::{ComponentHandle, SharedString, VecModel};
use sysinfo::{System, SystemExt};
use crate::cpu::{init_cpu, update_cpu};
use crate::ram::{init_ram, update_ram, update_swap};

use crate::storage::update_storage_data;
use crate::ui;
use crate::svg::generate_svg;
use crate::temperature::{display_components, init_temp, list_components, update_temp};
use crate::ui::{PieChartData, DiskData};

pub fn setup<T: Send + 'static>(window: &ui::Dashboard, receiver: Receiver<T>) -> JoinHandle<()> {
    let window_weak = window.as_weak();
    thread::spawn(move ||
        worker_loop(window_weak, receiver)
    )
}

fn worker_loop<T>(window_weak: slint::Weak<ui::Dashboard>, receiver: Receiver<T>) {
    let mut sys = System::new();
    let mut cpu_chart: Vec<Vec<f32>> = Vec::default();
    let mut ram_chart: Vec<f32> = Vec::default();
    let mut swap_chart: Vec<f32> = Vec::default();
    let mut temp_chart: Vec<Vec<f32>> = Vec::default();
    sys.refresh_disks_list();
    init_cpu(&mut sys, &mut cpu_chart);
    init_temp(&mut sys, &mut temp_chart);
    init_ram(&mut sys, &mut ram_chart, &mut swap_chart);
    display_components(&window_weak, list_components(&mut sys));
    loop {
        match receiver.try_recv() {
            Ok(_) => { break; }
            Err(_) => {}
        }
        update_cpu(&mut sys, &mut cpu_chart);
        update_temp(&mut sys, &mut temp_chart);
        let ram_pie_chart = update_ram(&mut sys, &mut ram_chart);
        let swap_pie_chart = update_swap(&mut sys, &mut swap_chart);
        let disks = update_storage_data(&mut sys);
        display_current(
            &window_weak, 
            cpu_chart.clone(), 
            temp_chart.clone(), 
            ram_chart.clone(), 
            ram_pie_chart, 
            swap_pie_chart,
            disks
        );
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

fn vec_to_vec_model<T: std::default::Default + 'static>(vec: Vec<T>) -> VecModel<T> {
    let model: VecModel<T> = VecModel::default();
    for value in vec {
        model.push(value)
    }
    return model;
}

fn chart_to_shared_string(chart: &Vec<f32>) -> SharedString {
    let chart_model = SharedString::from(generate_svg(&chart));
    return chart_model;
}

fn display_current(
    window_weak: &slint::Weak<ui::Dashboard>,
    cpu_chart: Vec<Vec<f32>>,
    temp_chart: Vec<Vec<f32>>,
    ram_chart: Vec<f32>,
    ram_pie_chart: PieChartData,
    swap_pie_chart: PieChartData,
    disks: Vec<DiskData>
) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            let cpu_model: VecModel<SharedString> = chart_to_chart_model(&cpu_chart);
            let temp_model: VecModel<SharedString> = chart_to_chart_model(&temp_chart);
            let ram_model: SharedString = chart_to_shared_string(&ram_chart);
            let stor_model = vec_to_vec_model(disks);
            let vm: ui::MainViewModel<'_> = window.global::<ui::MainViewModel>();
            vm.set_cpu_data(Rc::new(cpu_model).into());
            vm.set_temperature_data(Rc::new(temp_model).into());
            vm.set_ram_data(ram_model);
            vm.set_ram_pie_chart(ram_pie_chart);
            vm.set_swap_pie_chart(swap_pie_chart);
            vm.set_disks(Rc::new(stor_model).into());
        })
        .unwrap();
}

