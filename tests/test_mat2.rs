//! Test suite for the Web and headless browsers.
//
#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat2::{JsMatrix, Matrix};
use cao_math::vec::vec2::Point;
use cao_math::vectorization::Tensor2f;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_right_prod_2by2() {
    const SQRT3APROX: f32 = 1.73205080757;

    let mut mat = Matrix::default();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    let p = [1., 2.];
    let res = mat.right_prod(p);

    assert_eq!(res, [SQRT3APROX * 2., 3.]);
}

#[wasm_bindgen_test]
fn basic_left_prod_2by2() {
    const SQRT3APROX: f32 = 1.73205080757;

    let mut mat = Matrix::default();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    let p = [1., 2.];
    let res = mat.left_prod(p);

    assert_eq!(res, [SQRT3APROX, 3.8660254]);
}

#[wasm_bindgen_test]
fn basic_right_prod_2by2_tensor() {
    let mut tensor = Tensor2f::new();
    for _ in 0..512 {
        tensor.push(Point::new(1., 2.));
    }

    // scale by 2
    let mut mat = Matrix::default();
    mat.set(0, 0, 2.);
    mat.set(1, 0, 0.);
    mat.set(0, 1, 0.);
    mat.set(1, 1, 2.);

    let res = tensor.right_prod(&JsMatrix { val: mat });

    assert_eq!(tensor.data.len(), res.data.len());

    for (v1, v2) in tensor.data.iter().zip(res.data.iter()) {
        let [x1, y1]: [f32; 2] = v1.into();
        let [x2, y2]: [f32; 2] = v2.into();

        assert_eq!(x1 * 2., x2);
        assert_eq!(y1 * 2., y2);
    }
}
