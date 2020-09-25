use super::mat3::Mat33;
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Mat3f)]
#[derive(Debug, Clone)]
pub struct JsMat33 {
    #[wasm_bindgen(skip)]
    pub val: Mat33,
}

impl Default for JsMat33 {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class=Mat3f)]
impl JsMat33 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Default::default();
        Self { val }
    }

    /// Returns a flat list in column-major order
    #[wasm_bindgen(js_name=asList)]
    pub fn as_list(&self) -> Vec<f32> {
        let mut v = Vec::with_capacity(9);
        for c in 0..3 {
            for r in 0..3 {
                v.push(self.val.at(c, r));
            }
        }
        v
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.val.at(col, row)
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, a: f32) {
        self.val.set(col, row, a);
    }

    /// Calculate the `a*M` product
    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod_js(&self, a: &JsValue) -> Result<Vec3, JsValue> {
        let a: Vec3 = a
            .into_serde()
            .map_err(|err| format!("Failed to convert value into a vector {:?}", err))?;
        let a = self.val.left_prod(a.into());
        let a = Vec3::from(a);
        Ok(a)
    }

    /// Calculate the `M*a` product
    #[wasm_bindgen(js_name=rightProd)]
    pub fn right_prod_js(&self, a: &JsValue) -> Result<Vec3, JsValue> {
        let a: Vec3 = a
            .into_serde()
            .map_err(|err| format!("Failed to convert value into a vector {:?}", err))?;
        let a = self.val.right_prod(a.into());
        let a = Vec3::from(a);
        Ok(a)
    }

    /// Calculate the `a*M` product where `a` is a cao-math vec3
    #[wasm_bindgen(js_name=leftProdVec)]
    pub fn left_prod(&self, a: &Vec3) -> Vec3 {
        let a = self.val.left_prod(a.into());
        Vec3::from(a)
    }

    /// Calculate the `M*a` product where `a` is a cao-math vec3
    #[wasm_bindgen(js_name=rightProdVec)]
    pub fn right_prod(&self, a: &Vec3) -> Vec3 {
        let a = self.val.right_prod(a.into());
        Vec3::from(a)
    }

    #[wasm_bindgen(js_name=rotationMat33)]
    pub fn rotation(radians: f32) -> Self {
        Mat33::rotation(radians).into()
    }

    #[wasm_bindgen(js_name=scaleMat33)]
    pub fn scale(a: f32) -> Self {
        Mat33::scale(a).into()
    }

    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    #[wasm_bindgen(js_name=translateMat33)]
    pub fn translate(t: &Vec2) -> Self {
        Mat33::translate(t.into()).into()
    }

    /// Multiply by scalar value
    #[wasm_bindgen(js_name=scalarMul)]
    pub fn scalar_mul(&mut self, f: f32) {
        self.val *= f;
    }

    /// Multiply two matrices
    #[wasm_bindgen(js_name=matrixMul)]
    pub fn mul(&self, b: &JsMat33) -> JsMat33 {
        let mut res = Mat33::default();
        self.val.mat_mul(&b.val, &mut res);
        res.into()
    }
}

impl From<Mat33> for JsMat33 {
    fn from(val: Mat33) -> Self {
        Self { val }
    }
}
