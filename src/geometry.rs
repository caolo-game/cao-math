use crate::hex::cube_distance;
use crate::vec::vec3::Vec3;
use crate::vectorization::Tensor3f;
use wasm_bindgen::prelude::*;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[wasm_bindgen(js_name= cubeLerp)]
pub fn cube_lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    [lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t)].into()
}

/// Return a list of points, each point will be inside a hex that is intersected by the line
/// between points `a` and `b`
#[wasm_bindgen(js_name= cubeLinePoints)]
pub fn cube_line_points(a: &Vec3, b: &Vec3) -> Tensor3f {
    let n = cube_distance(a, b);

    let data = (0..n)
        .map(|i| cube_lerp(a, b, 1.0 / (n as f32) * i as f32))
        .collect();

    Tensor3f { data }
}
