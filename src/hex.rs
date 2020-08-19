//! Hex-Grid utilities
use crate::mat::mat2::{JsMatrix, Matrix};
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use wasm_bindgen::prelude::*;

const SQRT3APROX: f32 = 1.732_050_807_57;

/// Converts a point on the hex grid from `axial` representation to `cube` representation
#[wasm_bindgen(js_name = axialToCube)]
pub fn axial_to_cube(axial: &Vec2) -> Vec3 {
    let x = axial.x;
    let z = axial.y;
    let y = -x - z;
    [x, y, z].into()
}

/// 'Manhatten-esque' distance between two 'cube' points on the hex grid
#[wasm_bindgen(js_name = cubeDistance)]
pub fn cube_distance(a: &Vec3, b: &Vec3) -> u32 {
    let x = (a.x - b.x).abs();
    let y = (a.y - b.y).abs();
    let z = (a.z - b.z).abs();
    x.max(y).max(z) as u32
}

/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "pointy top grid".
#[wasm_bindgen(js_name = axialToPixelMatrixPointy)]
pub fn axial_to_pixel_mat_pointy() -> JsMatrix {
    let mat: Matrix = [[SQRT3APROX, SQRT3APROX / 2.0], [0., 3. / 2.]].into();
    mat.into()
}

/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "flat top grid".
#[wasm_bindgen(js_name = axialToPixelMatrixFlat)]
pub fn axial_to_pixel_mat_flat() -> JsMatrix {
    let mat: Matrix = [[3. / 2., 0.], [SQRT3APROX / 2., SQRT3APROX]].into();
    mat.into()
}

/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "pointy top grid".
#[wasm_bindgen(js_name = pixelToAxialMatrixPointy)]
pub fn pixel_to_axial_pointy() -> JsMatrix {
    let mat: Matrix = [[SQRT3APROX / 3., -1. / 3.], [0., 2. / 3.]].into();
    mat.into()
}

/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "flat top grid".
#[wasm_bindgen(js_name = pixelToAxialMatrixFlat)]
pub fn pixel_to_axial_flat() -> JsMatrix {
    let mat: Matrix = [[2. / 3., 0.], [-1. / 3., SQRT3APROX / 3.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = roundToNearestAxial)]
pub fn round_to_nearest_axial(axial: &Vec2) -> Vec2 {
    // convert to cube
    let [x, z]: [f32; 2] = axial.into();
    let y = -x - z;
    // round
    let [mut rx, ry, mut rz] = [x.round(), y.round(), z.round()];
    let [dx, dy, dz] = [(rx - x).abs(), (ry - y).abs(), (rz - z).abs()];
    if dx > dy && dx > dz {
        rx = -ry - rz;
    } else if dy <= dz {
        rz = -rx - ry;
    }
    // convert back to axial
    Vec2::new(rx, rz)
}
