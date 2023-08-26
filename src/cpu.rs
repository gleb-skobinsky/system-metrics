use sysinfo::{ComponentExt, CpuExt, System, SystemExt};

pub fn update_cpu(sys: &mut System, cpu_chart: &mut Vec<Vec<f32>>) {
    sys.refresh_cpu();
    let cpus = sys.cpus();
    for (i, cpu) in cpus.iter().enumerate() {
        let usage = cpu.cpu_usage();
        cpu_chart[i].remove(0);
        cpu_chart[i].push(usage);
    }
}

pub fn init_cpu(sys: &mut System, cpu_chart: &mut Vec<Vec<f32>>) {
    sys.refresh_cpu();
    for cpu in sys.cpus() {
        let mut vector: Vec<f32> = Vec::with_capacity(20);
        let usage = cpu.cpu_usage();
        for _ in 0..20 {
            vector.push(usage);
        }
        cpu_chart.push(vector);
    }
}