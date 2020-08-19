#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat3::Matrix;
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

#[wasm_bindgen_test]
fn basic_mat_mat_3_ab() {
    let a = Matrix::from([[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]]);
    let b = Matrix::from([[5., 6., 7.], [5., 6., 7.], [5., 6., 7.]]);

    let mut c = Matrix::default();

    a.mat_mul(&b, &mut c);

    assert_eq!(
        c.values,
        [[30., 36., 42.], [30., 36., 42.], [30., 36., 42.],]
    );
}

#[wasm_bindgen_test]
fn basic_mat_mat_3_ba() {
    let a = Matrix::from([[5., 6., 7.], [5., 6., 7.], [5., 6., 7.]]);
    let b = Matrix::from([[1., 2., 3.], [1., 2., 3.], [1., 2., 3.]]);

    let mut c = Matrix::default();

    a.mat_mul(&b, &mut c);

    assert_eq!(
        c.values,
        [[18., 36., 54.], [18., 36., 54.], [18., 36., 54.]]
    );
}

#[wasm_bindgen_test]
fn test_translation() {
    let a = Matrix::translate([1., 2.]);

    let p = [0., 0., 1.];

    let res = a.right_prod(p);

    assert_eq!(res, [1., 2., 1.]);
}

#[wasm_bindgen_test]
fn basic_mat_multiplication() {
    let a = Matrix::translate([5.0, 6.0]);
    let b = Matrix::scale(8.0);

    let mut c = Matrix::default();
    a.mat_mul(&b, &mut c);

    let control = Matrix::from([[8., 0., 40.], [0., 8., 48.], [0., 0., 8.]]);

    assert_eq!(c, control);
}
