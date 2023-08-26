use std::rc::Rc;
use slint::{ComponentHandle, SharedString, VecModel};
use sysinfo::{ComponentExt, System, SystemExt};
use crate::ui;

pub fn init_temp(sys: &mut System, temperature_chart: &mut Vec<Vec<f32>>) {
    sys.refresh_components_list();
    sys.refresh_components();
    let sensor_data = sys.components();
    for component in sensor_data {
        let mut vector: Vec<f32> = Vec::with_capacity(20);
        let temperature = component.temperature();
        for _ in 0..20 {
            vector.push(temperature);
        }
        temperature_chart.push(vector);
    }
}

pub fn list_components(sys: &mut System) -> Vec<String> {
    sys.refresh_components_list();
    let components = sys.components().iter().map(|component|
        component.label().to_string()
    ).collect();
    return components;
}

fn vec_to_vec_model(values: Vec<String>) -> VecModel<SharedString> {
    let mut model = VecModel::default();
    for value in values {
        model.push(SharedString::from(value))
    }
    return model
}

pub fn update_temp(sys: &mut System, temperature_chart: &mut Vec<Vec<f32>>) {
    sys.refresh_components();
    let sensor_data = sys.components();
    for (i, component) in sensor_data.iter().enumerate() {
        let temperature = component.temperature();
        temperature_chart[i].remove(0);
        temperature_chart[i].push(temperature);
    }
}

pub fn display_components(
    window_weak: &slint::Weak<ui::Dashboard>,
    components: Vec<String>
) {
    window_weak
        .upgrade_in_event_loop(move |window| {
            let vm = window.global::<ui::MainViewModel>();
            vm.set_cpu_data(Rc::new(vec_to_vec_model(components)).into());
        })
        .unwrap();
}