//! Hex-Grid utilities
use crate::mat::mat2f32;
use wasm_bindgen::prelude::*;

const SQRT3APROX: f32 = 1.73205080757;

#[wasm_bindgen(js_name = axialToPixelMatrixPointy)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "pointy top grid"
pub fn axial_to_pixel_mat_pointy() -> mat2f32::Proxy {
    let mut mat = mat2f32::Matrix::default();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    mat.into()
}

#[wasm_bindgen(js_name = axialToPixelMatrixFlat)]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "flat top grid"
pub fn axial_to_pixel_mat_flat() -> mat2f32::Proxy {
    let mut mat = mat2f32::Matrix::default();

    mat.set(0, 0, 3.0 / 2.0);
    mat.set(1, 0, 0.0);
    mat.set(0, 1, SQRT3APROX / 2.0);
    mat.set(1, 1, SQRT3APROX);

    mat.into()
}
