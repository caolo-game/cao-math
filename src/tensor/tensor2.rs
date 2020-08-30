use crate::mat::js_mat2::JsMat22;
use crate::vec::vec2::Vec2;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Collection of 2d float vectors
#[wasm_bindgen(js_name=Tensor2f, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor2f {
    #[wasm_bindgen(skip)]
    pub data: Vec<Vec2>,
}

#[wasm_bindgen(js_class=Tensor2f)]
impl Tensor2f {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    #[wasm_bindgen]
    pub fn push(&mut self, v: Vec2) {
        self.data.push(v);
    }

    #[wasm_bindgen]
    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }

    #[wasm_bindgen]
    pub fn get(&self, i: usize) -> Vec2 {
        self.data[i]
    }

    #[wasm_bindgen]
    pub fn set(&mut self, i: usize, v: Vec2) {
        *self.data.get_mut(i).expect("Invalid index") = v;
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, i: usize) -> Vec2 {
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
    pub fn right_prod(&mut self, m: &JsMat22) {
        self.data.iter_mut().for_each(|v| *v = m.right_prod(v));
    }

    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod(&mut self, m: &JsMat22) {
        self.data.iter_mut().for_each(|v| *v = m.left_prod(v));
    }
}
