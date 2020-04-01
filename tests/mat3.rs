#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat3f32::Matrix;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_left_prod_3by3() {
    let mut mat = Matrix::scale(1.);
    mat.set(1, 0, 2.);

    let p = [1., 2., 3.];
    let res = mat.left_prod(p);

    assert_eq!(res, [1., 4., 3.]);
}

#[wasm_bindgen_test]
fn basic_right_prod_3by3() {
    let mut mat = Matrix::scale(1.);
    mat.set(1, 0, 2.);

    let p = [1., 2., 3.];
    let res = mat.right_prod(p);

    assert_eq!(res, [5., 2., 3.]);
}
