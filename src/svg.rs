pub fn generate_svg(values: &[f32]) -> String {
    let mut svg = String::new();
    let mut x = 0;

    for (i, value) in values.iter().enumerate() {
        let mut mark = "L";
        if i == 0 {
            mark = "M"
        }
        svg.push_str(&format!("{} {} {} ", mark, x, -value));
        x += 10;
    }
    svg
}