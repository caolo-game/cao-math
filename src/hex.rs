//! Hex-Grid utilities
use crate::mat::mat2f32::{Matrix, Proxy};
use wasm_bindgen::prelude::*;

const SQRT3APROX: f32 = 1.73205080757;

#[wasm_bindgen(js_name = axialToPixelMatrixPointy)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "pointy top grid"
pub fn axial_to_pixel_mat_pointy() -> Proxy {
    let mat: Matrix = [[SQRT3APROX, SQRT3APROX / 2.0], [0., 3. / 2.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = axialToPixelMatrixFlat)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "flat top grid"
pub fn axial_to_pixel_mat_flat() -> Proxy {
    let mat: Matrix = [[3. / 2., 0.], [SQRT3APROX / 2., SQRT3APROX]].into();
    mat.into()
}
