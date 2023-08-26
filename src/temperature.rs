use sysinfo::{ComponentExt, System, SystemExt};

pub fn init_temp(sys: &mut System, temperature_chart: &mut Vec<Vec<f32>>) {
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

pub fn update_temp(sys: &mut System, temperature_chart: &mut Vec<Vec<f32>>) {
    let sensor_data = sys.components();
    for (i, component) in sensor_data.iter().enumerate() {
        let temperature = component.temperature();
        temperature_chart[i].remove(0);
        temperature_chart[i].push(temperature);
    }
}