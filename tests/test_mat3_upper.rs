#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat3::Matrix;
use cao_math::mat::mat3_upper::UpperMatrix;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_translation() {
    let a = UpperMatrix::translate([1., 2.]);

    let p = [0., 0., 1.];

    let res = a.right_prod(p);

    assert_eq!(res, [1., 2., 1.]);
}

#[wasm_bindgen_test]
fn test_matrix_conversion() {
    let mut a = UpperMatrix::default();
    a.set(0, 0, 1.);
    a.set(1, 0, 2.);
    a.set(2, 0, 3.);
    a.set(1, 1, 4.);
    a.set(2, 1, 5.);
    a.set(2, 2, 6.);

    let b =
        UpperMatrix::from_mat(&Matrix::from([[1., 2., 3.], [0., 4., 5.], [0., 0., 6.]])).unwrap();

    assert_eq!(a, b);
}

#[wasm_bindgen_test]
fn test_matrix_conversion_err() {
    UpperMatrix::from_mat(&Matrix::from([[1., 2., 3.], [1., 4., 5.], [0., 0., 6.]])).unwrap_err();
    UpperMatrix::from_mat(&Matrix::from([[1., 2., 3.], [0., 4., 5.], [1., 0., 6.]])).unwrap_err();
    UpperMatrix::from_mat(&Matrix::from([[1., 2., 3.], [0., 4., 5.], [0., 1., 6.]])).unwrap_err();
}
