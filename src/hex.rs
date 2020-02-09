//! Hex-Grid utilities
use crate::mat::mat2f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "axialToPixelMatrix")]
/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`
pub fn axial_to_pixel_mat() -> mat2f32::Proxy {
    const SQRT3APROX: f32 = 1.73205080757;

    let mut mat = mat2f32::Matrix::default();
    *mat.at_mut(0, 0) = SQRT3APROX;
    *mat.at_mut(1, 0) = SQRT3APROX / 2.0;
    *mat.at_mut(0, 1) = 0.0;
    *mat.at_mut(1, 1) = 3.0 / 2.0;

    mat.into()
}
