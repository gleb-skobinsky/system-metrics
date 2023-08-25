/*
pub fn average(numbers: Vec<f32>) -> f32 {
    let nnumbers = numbers.len() as f32;
    let mut sum = 0.0;
    for n in numbers {
        sum += n;
    }
    sum / nnumbers
}

fn get_cpu_usage(sys: &mut System) -> f32 {
    sys.refresh_cpu();
    let mut usage: Vec<f32> = Vec::new();
    for cpu in sys.cpus() {
        usage.push(cpu.cpu_usage());
    }
    return average(usage);
}
*/