#![cfg(target_arch = "wasm32")]

use cao_math::hex::{self, Hexagon};
use cao_math::vec::vec2::Vec2;
use wasm_bindgen_test::*;

const SIZE: f32 = 12.0;

#[wasm_bindgen_test]
fn test_pixel_to_axial_pointy() {
    let mat = hex::pixel_to_axial_pointy();

    let pix_coord = Vec2::new(3.14 * SIZE, -2.3 * SIZE);

    let Vec2 { x: hx, y: hy } = mat.right_prod(pix_coord.into());
    let hex_coord = hex::round_to_nearest_axial(&Vec2::new(hx / SIZE, hy / SIZE));

    assert_eq!(hex_coord, Vec2::new(3., -2.));
}

#[wasm_bindgen_test]
fn test_list_points_r_eq_3() {
    const R: f32 = 3.0;

    let hex = Hexagon::from_radius(R);

    let pts = hex.list_points();
    let pts = pts
        .into_iter()
        .map(|p| p.into_serde().unwrap())
        .collect::<Vec<[i32; 2]>>();

    assert_eq!(
        pts,
        vec![
            [0, 6],
            [0, 5],
            [0, 4],
            [0, 3],
            [1, 6],
            [1, 5],
            [1, 4],
            [1, 3],
            [1, 2],
            [2, 6],
            [2, 5],
            [2, 4],
            [2, 3],
            [2, 2],
            [2, 1],
            [3, 6],
            [3, 5],
            [3, 4],
            [3, 3],
            [3, 2],
            [3, 1],
            [3, 0],
            [4, 5],
            [4, 4],
            [4, 3],
            [4, 2],
            [4, 1],
            [4, 0],
            [5, 4],
            [5, 3],
            [5, 2],
            [5, 1],
            [5, 0],
            [6, 3],
            [6, 2],
            [6, 1],
            [6, 0],
        ]
    )
}
