pub fn generate_svg(values: &[f32]) -> String {
    let mut svg = String::new();
    let mut x = 0;

    for (i, value) in values.iter().enumerate() {
        let command: String;
        if i == 0 {
            command = format!("M {} {} ", x, -value);
        } else {
            let prev_y = values[i - 1];
            let middle_x = x - 5;
            command = format!("C {} {} {} {} {} {} ", middle_x, -prev_y, middle_x, -value, x, -value)
        }
        svg.push_str(&command);
        x += 10;
    }
    svg
}