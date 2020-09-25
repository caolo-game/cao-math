#![cfg(target_arch = "wasm32")]

use cao_math::vec::vec2::Vec2;
use std::f32::{consts::PI, EPSILON};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn perform_angle_test(u: [f32; 2], v: [f32; 2], exp: f32) {
    let u: Vec2 = u.into();
    let v: Vec2 = v.into();

    let ang = u.angle_between(&v);
    let diff = ang - exp;

    assert!(diff <= EPSILON);
}

#[wasm_bindgen_test]
fn test_angle_basic() {
    perform_angle_test([1.0, 0.0], [0.0, 1.0], PI / 2.0);
    perform_angle_test([0.0, 1.0], [1.0, 0.0], PI / 2.0);
    perform_angle_test([1.0, 0.0], [1.0, 0.0], 0.0);
    perform_angle_test([0.0, 1.0], [0.0, 1.0], 0.0);
    perform_angle_test([0.13, -4.2], [-0.13, 4.2], PI);
}
