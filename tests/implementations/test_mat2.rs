//! Test suite for the Web and headless browsers.
//
#![cfg(target_arch = "wasm32")]

use cao_math::mat::mat2::Mat2f;
use cao_math::tensor::Tensor2f;
use cao_math::vec::vec2::Vec2;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn basic_right_prod_2by2() {
    const SQRT3APROX: f32 = 1.73205080757;

    let mut mat = Mat2f::default();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    let p = [1., 2.].into();
    let res = mat.right_prod(p);

    assert_eq!(res, [SQRT3APROX * 2., 3.].into());
}

#[wasm_bindgen_test]
fn basic_left_prod_2by2() {
    const SQRT3APROX: f32 = 1.73205080757;

    let mut mat = Mat2f::default();
    mat.set(0, 0, SQRT3APROX);
    mat.set(1, 0, SQRT3APROX / 2.0);
    mat.set(0, 1, 0.0);
    mat.set(1, 1, 3.0 / 2.0);

    let p = [1., 2.].into();
    let res = mat.left_prod(p);

    assert_eq!(res, [SQRT3APROX, 3.8660254].into());
}

#[wasm_bindgen_test]
fn basic_right_prod_2by2_tensor() {
    let mut tensor = Tensor2f::new();
    for _ in 0..512 {
        tensor.push(Vec2::new(1., 2.));
    }

    let mat = Mat2f::scale(2.0);

    let expected = tensor
        .data
        .iter()
        .map(|v| {
            let [x, y]: [f32; 2] = v.into();
            [x * 2., y * 2.]
        })
        .collect::<Vec<_>>();

    tensor.right_prod(&mat);

    for (v1, v2) in tensor.data.iter().zip(expected.iter()) {
        let [x1, y1]: [f32; 2] = v1.into();
        let [x2, y2]: [f32; 2] = *v2;

        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }
}

#[wasm_bindgen_test]
fn inverse_returns_correct_result() {
    let mat = Mat2f {
        x_axis: [4., 7.],
        y_axis: [2., 6.],
    }
    .transposed();

    let exp = Mat2f {
        x_axis: [0.6, -0.7],
        y_axis: [-0.2, 0.4],
    }
    .transposed();

    let inv = mat.inverted().expect("expected invert to succeed");

    assert_eq!(inv, exp);
    let id = &inv * mat;
    assert!(
        id.almost_equal(&Mat2f::identity(), 0.000001),
        "inverse: {:?}\'identity': {:?}",
        inv,
        id
    );
}

#[wasm_bindgen_test]
fn inverse_returns_none_is_non_invertible() {
    let mat = Mat2f {
        x_axis: [0., 4.],
        y_axis: [0., -2.],
    };

    assert!(
        mat.inverted().is_none(),
        "matrix with 0 row is not invertible"
    );

    let mat = Mat2f {
        x_axis: [0., 0.],
        y_axis: [1., -2.],
    };

    assert!(
        mat.inverted().is_none(),
        "matrix with 0 column is not invertible"
    );
}
