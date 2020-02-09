//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat2f32;
use cao_math::point::vec2f32;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_right_prod() {
    const SQRT3APROX: f32 = 1.73205080757;

    let points =
        vec![JsValue::from_serde(&vec2f32::Point::new(1., 2.)).unwrap(); 25].into_boxed_slice();

    let mut mat = mat2f32::Proxy::new();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    let res = mat.right_prod(points).unwrap();
    assert!(res.is_object());
    let res: Box<[vec2f32::Point]> = res.into_serde().unwrap();
    for v in res.iter() {
        assert_eq!(v.x, SQRT3APROX * 2.);
        assert_eq!(v.y, 3.);
    }
}
