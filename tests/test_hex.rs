#![cfg(target_arch = "wasm32")]

use cao_math::hex;
use cao_math::vec::vec2::Vec2;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SIZE: f32 = 12.0;

#[wasm_bindgen_test]
fn test_pixel_to_axial_pointy() {
    let mat = hex::pixel_to_axial_pointy().val;

    let pix_coord = Vec2::new(3.14 * SIZE, -2.3 * SIZE);

    let [hx, hy] = mat.right_prod(pix_coord.into());
    let hex_coord = hex::round_to_nearest_axial(&Vec2::new(hx / SIZE, hy / SIZE));

    assert_eq!(hex_coord, Vec2::new(3., -2.));
}
