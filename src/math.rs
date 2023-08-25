pub fn average(numbers: Vec<f32>) -> f32 {
    let nnumbers = numbers.len() as f32;
    let mut sum = 0.0;
    for n in numbers {
        sum += n;
    }
    sum / nnumbers
}