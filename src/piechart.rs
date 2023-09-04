
const RADIUS: f32 = 100.0;
const CENTER_X: f32 = 100.0;
const CENTER_Y: f32 = 100.0;
use std::f64::consts::PI;

pub fn calculate_point_coordinates(angle: f32) -> (f32, f32) {
    let x = CENTER_X + RADIUS * angle.cos();
    let y = CENTER_Y + RADIUS * angle.sin();
    (x, y)
}

pub fn ratio_to_radians(ratio: f32) -> f32 {
    return ratio * 2.0 * (PI as f32)
}
