#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat3::Mat3f;
use std::f32::EPSILON;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_left_prod_3by3() {
    let mut mat = Mat3f::scale(1.);
    mat.set(1, 0, 2.);

    let p = [1., 2., 3.].into();
    let res = mat.left_prod(&p);

    assert_eq!(res, [1., 4., 3.].into());
}

#[wasm_bindgen_test]
fn basic_right_prod_3by3() {
    let mut mat = Mat3f::scale(1.);
    mat.set(1, 0, 2.);

    let p = [1., 2., 3.].into();
    let res = mat.right_prod(&p);

    assert_eq!(res, [5., 2., 3.].into());
}

#[wasm_bindgen_test]
fn basic_mat_mat_3_ab() {
    let a = Mat3f::from([[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]]);
    let b = Mat3f::from([[5., 6., 7.], [5., 6., 7.], [5., 6., 7.]]);

    let c = a.mat_mul(&b);

    assert_eq!(c.x_axis, [18., 36., 54.]);
    assert_eq!(c.y_axis, [18., 36., 54.]);
    assert_eq!(c.w_axis, [18., 36., 54.]);
}

#[wasm_bindgen_test]
fn basic_mat_mat_3_ba() {
    let a = Mat3f::from([[5., 6., 7.], [5., 6., 7.], [5., 6., 7.]]);
    let b = Mat3f::from([[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]]);

    let c = a.mat_mul(&b);

    assert_eq!(c.x_axis, [30.0, 36.0, 42.0]);
    assert_eq!(c.y_axis, [30.0, 36.0, 42.0]);
    assert_eq!(c.w_axis, [30.0, 36.0, 42.0]);
}

#[wasm_bindgen_test]
fn test_translation() {
    let a = Mat3f::translate([1., 2.].into());

    let p = [0., 0., 1.].into();

    let res = a.right_prod(&p);

    assert_eq!(res, [1., 2., 1.].into());
}

#[wasm_bindgen_test]
fn basic_mat_multiplication() {
    let a = Mat3f::translate([5.0, 6.0].into());
    let b = Mat3f::scale(8.0);

    let c = a.mat_mul(&b);

    let control = Mat3f {
        x_axis: [8.0, 0.0, 0.0],
        y_axis: [0.0, 8.0, 0.0],
        w_axis: [40.0, 48.0, 8.0],
    };

    assert_eq!(c, control);
}

#[wasm_bindgen_test]
fn determinant() {
    let a = Mat3f {
        x_axis: [6., 4., 2.],
        y_axis: [1., -2., 8.],
        w_axis: [1., 5., 7.],
    };

    let d = a.det();

    assert!((d - -306.).abs() < EPSILON, "{}", d);
}
