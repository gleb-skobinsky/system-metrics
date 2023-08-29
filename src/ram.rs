use sysinfo::{System, SystemExt};

pub fn init_ram(sys: &mut System, ram_chart: &mut Vec<f32>) {
    sys.refresh_memory();      
    let percentage = get_ram_ratio(&sys);
    for _ in 0..20 {
        ram_chart.push(percentage);
    }    
}

pub fn update_ram(sys: &mut System, ram_chart: &mut Vec<f32>) {
    sys.refresh_memory();
    let percentage = get_ram_ratio(&sys);
    ram_chart.remove(0);
    ram_chart.push(percentage);
}

fn get_ram_ratio(sys: &System) -> f32 {
    let used_ram = sys.used_memory() as f32;
    let total_ram = sys.total_memory() as f32;
    return (used_ram / total_ram) * 100.0;
}