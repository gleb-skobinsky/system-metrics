use sysinfo::{System, SystemExt};
use crate::{ui::PieChartData, piechart::{calculate_point_coordinates, ratio_to_radians}};

fn get_ram_ratio(sys: &System) -> f32 {
    let used_ram = sys.used_memory() as f32;
    let total_ram = sys.total_memory() as f32;
    return used_ram / total_ram;
}

fn get_swap_ratio(sys: &System) -> f32 {
    let used_swap = sys.used_swap() as f32;
    let total_swap = sys.total_swap() as f32;
    return used_swap / total_swap;
}

pub fn init_ram(sys: &mut System, ram_chart: &mut Vec<f32>, swap_chart: &mut Vec<f32>) {
    sys.refresh_memory();
    let percentage = get_ram_ratio(&sys);
    let swap_ratio = get_swap_ratio(&sys);
    for _ in 0..20 {
        ram_chart.push(percentage * 100.0);
        swap_chart.push(swap_ratio * 100.0)
    }    
}

pub fn update_ram(sys: &mut System, ram_chart: &mut Vec<f32>) -> PieChartData {
    sys.refresh_memory();
    let ratio = get_ram_ratio(&sys);
    
    ram_chart.remove(0);
    ram_chart.push(ratio * 100.0);
    
    let (x, y) = calculate_point_coordinates(ratio_to_radians(ratio));
    return PieChartData {
        percentage: round2(ratio * 100.0),
        angleX: x as i32,
        angleY: y as i32
    };
}

pub fn update_swap(sys: &mut System, swap_chart: &mut Vec<f32>) -> PieChartData {
    sys.refresh_memory();
    let swap = get_swap_ratio(&sys);
    swap_chart.remove(0);
    swap_chart.push(swap * 100.0);
    let (x, y) = calculate_point_coordinates(ratio_to_radians(swap));
    return PieChartData {
        percentage: round2(swap * 100.0),
        angleX: x as i32,
        angleY: y as i32
    };
}


fn round2(value: f32) -> f32 {
    return (value * 100.0).round() / 100.0;
}