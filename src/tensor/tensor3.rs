use crate::mat::mat3::Mat3f;
use crate::vec::vec3::Vec3;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Collection of 3d float vectors
#[wasm_bindgen(js_name=Tensor3f, inspectable)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor3f {
    #[wasm_bindgen(skip)]
    pub data: Vec<Vec3>,
}

#[wasm_bindgen(js_class=Tensor3f)]
impl Tensor3f {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Extends this instance with a list of `{ x, y, z }` points.
    ///
    /// Invalid items will be dropped
    #[wasm_bindgen]
    pub fn extend(&mut self, list: Box<[JsValue]>) {
        self.data.extend(
            list.iter()
                .filter(|js| js.is_object())
                .filter_map(|js| js.into_serde::<Vec3>().ok()),
        )
    }

    #[wasm_bindgen]
    pub fn push(&mut self, v: Vec3) {
        self.data.push(v);
    }

    #[wasm_bindgen]
    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }

    #[wasm_bindgen]
    pub fn get(&self, i: usize) -> Vec3 {
        self.data[i]
    }

    #[wasm_bindgen]
    pub fn set(&mut self, i: usize, v: Vec3) {
        *self.data.get_mut(i).expect("Invalid index") = v;
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, i: usize) -> Vec3 {
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

    /// Perform `M*v` for each `v` vector in this tensor
    #[wasm_bindgen(js_name=rightProd)]
    pub fn right_prod(&mut self, m: &Mat3f) {
        self.data.iter_mut().for_each(|v| *v = m.right_prod(v));
    }

    /// Perform `v*M` for each `v` vector in this tensor
    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod(&mut self, m: &Mat3f) {
        self.data.iter_mut().for_each(|v| *v = m.left_prod(v));
    }
}
