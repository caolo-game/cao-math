use crate::mat::mat3f32::JsMatrix;
use crate::vec::vec3f32::Point;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Collection of 3d float vectors
#[wasm_bindgen(js_name=Tensor3f, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor3f {
    #[wasm_bindgen(skip)]
    pub data: Vec<Point>,
}

#[wasm_bindgen(js_class=Tensor3f)]
impl Tensor3f {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    #[wasm_bindgen]
    pub fn push(&mut self, v: Point) {
        self.data.push(v);
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, i: usize) -> Point {
        self.data.remove(i)
    }

    /// Turn this instance into a list of vectors
    #[wasm_bindgen(js_name=toList)]
    pub fn to_list(&self) -> Box<[JsValue]> {
        self.data
            .iter()
            .filter_map(|p| JsValue::from_serde(&p).ok())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(js_name=rightProd)]
    pub fn right_prod(&self, m: &JsMatrix) -> Self {
        Self {
            data: self.data.iter().map(|v| m.right_prod(v)).collect(),
        }
    }

    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod(&self, m: &JsMatrix) -> Self {
        Self {
            data: self.data.iter().map(|v| m.left_prod(v)).collect(),
        }
    }
}
