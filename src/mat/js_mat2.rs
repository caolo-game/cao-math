use super::mat2::Mat22;

use super::js_mat3::JsMat33;
use crate::vec::vec2::Vec2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Mat2f)]
pub struct JsMat22 {
    #[wasm_bindgen(skip)]
    pub val: Mat22,
}

#[wasm_bindgen(js_class=Mat2f)]
impl JsMat22 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Mat22::default();
        Self { val }
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.val.at(col, row)
    }

    /// Returns a flat list in column-major order
    #[wasm_bindgen(js_name=asList)]
    pub fn as_list(&self) -> Vec<f32> {
        let mut v = Vec::with_capacity(9);
        v.extend_from_slice(&self.val.x_axis);
        v.extend_from_slice(&self.val.y_axis);
        v
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, a: f32) {
        *self.val.at_mut(col, row) = a;
    }

    #[wasm_bindgen(js_name=leftProd)]
    /// Calculate the `a*M` product
    pub fn left_prod(&self, a: &Vec2) -> Vec2 {
        let a = self.val.left_prod(a.into());
        Vec2::from(a)
    }

    #[wasm_bindgen(js_name=rightProd)]
    /// Calculate the `M*a` product
    pub fn right_prod(&self, a: &Vec2) -> Vec2 {
        let a = self.val.right_prod(a.into());
        Vec2::from(a)
    }

    #[wasm_bindgen(js_name=scaleMatrix)]
    pub fn scale(a: f32) -> Self {
        Mat22::scale(a).into()
    }

    /// Create a 3x3 matrix from `this=A`.
    /// ```txt
    /// | a00 a10 0 |
    /// | a01 a11 0 |
    /// |   0   0 1 |
    /// ```
    #[wasm_bindgen(js_name=asMat3f)]
    pub fn as_mat3f(&self) -> JsMat33 {
        let [a00, a01] = self.val.x_axis;
        let [a10, a11] = self.val.y_axis;
        JsMat33 {
            val: [[a00, a01, 0.], [a10, a11, 0.], [0., 0., 1.]].into(),
        }
    }
}

impl From<Mat22> for JsMat22 {
    fn from(val: Mat22) -> Self {
        Self { val }
    }
}
