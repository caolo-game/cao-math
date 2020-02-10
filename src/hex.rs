//! Hex-Grid utilities
use crate::mat::mat2f32::{Matrix, Proxy};
use crate::point::vec2f32::Point;
use wasm_bindgen::prelude::*;

const SQRT3APROX: f32 = 1.73205080757;

#[wasm_bindgen(js_name = axialToPixelMatrixPointy)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "pointy top grid".
pub fn axial_to_pixel_mat_pointy() -> Proxy {
    let mat: Matrix = [[SQRT3APROX, SQRT3APROX / 2.0], [0., 3. / 2.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = axialToPixelMatrixFlat)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "flat top grid".
pub fn axial_to_pixel_mat_flat() -> Proxy {
    let mat: Matrix = [[3. / 2., 0.], [SQRT3APROX / 2., SQRT3APROX]].into();
    mat.into()
}

#[wasm_bindgen(js_name = pixelToAxialMatrixPointy)]
/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "pointy top grid".
pub fn pixel_to_axial_pointy() -> Proxy {
    let mat: Matrix = [[SQRT3APROX / 3., -1. / 3.], [0., 2. / 3.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = pixelToAxialMatrixFlat)]
/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "flat top grid".
pub fn pixel_to_axial_flat() -> Proxy {
    let mat: Matrix = [[2. / 3., 0.], [-1. / 3., SQRT3APROX / 3.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = roundToNearestHex)]
pub fn round_to_nearest(axial: &Point) -> Point {
    // convert to cube
    let [x, z]: [f32; 2] = axial.into();
    let y = -x - z;
    let [mut rx, ry, mut rz] = [x.round(), y.round(), z.round()];
    let [dx, dy, dz] = [(rx - x).abs(), (ry - y).abs(), (rz - z).abs()];

    if dx > dy && dx > dz {
        rx = -ry - rz;
    } else if dy <= dz {
        rz = -rx - ry;
    }

    // convert back to axial
    let x = rx;
    let y = rz;
    Point::new(x, y)
}
