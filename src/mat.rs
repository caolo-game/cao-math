//! Basic fix sized matrices
use crate::point::*;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use wasm_bindgen::prelude::*;

macro_rules! impl_square_mat {
    ($mat: ident, $val: ty, $dim: expr, $vec: ty, $proxy: ident) => {
        pub mod $mat {
            use super::*;

            #[derive(Debug, Clone, Default, Serialize, Deserialize)]
            pub struct Matrix {
                pub values: [$val; $dim * $dim],
            }

            #[wasm_bindgen(js_name=$proxy)]
            pub struct Proxy {
                val: Matrix,
            }

            #[wasm_bindgen(js_class=$proxy)]
            impl Proxy {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Self {
                    let val = Matrix::default();
                    Self { val }
                }

                #[wasm_bindgen]
                pub fn at(&self, col: usize, row: usize) -> $val {
                    self.val.at(col, row)
                }

                #[wasm_bindgen]
                pub fn set(&mut self, col: usize, row: usize, a: $val) {
                    *self.val.at_mut(col, row) = a;
                }

                #[wasm_bindgen(js_name=leftProd)]
                /// Calculate the `a*M` product for all vector `a` in `vecs`
                pub fn left_prod(&self, vecs: Box<[JsValue]>) -> Result<JsValue, JsValue> {
                    let mut res = Vec::new();
                    for v in vecs.into_iter() {
                        let v: $vec = serde_wasm_bindgen::from_value(v.clone())?;
                        let v = self.val.left_prod(v.into());
                        let v = <$vec>::from(v);
                        res.push(v);
                    }
                    let res = res.into_boxed_slice();
                    let res = JsValue::from_serde(&res).unwrap();
                    Ok(res)
                }

                #[wasm_bindgen(js_name=rightProd)]
                /// Calculate the `M*a` product for all vector `a` in `vecs`
                pub fn right_prod(&self, vecs: Box<[JsValue]>) -> Result<JsValue, JsValue> {
                    let mut res = Vec::new();
                    for v in vecs.into_iter() {
                        let v: $vec = serde_wasm_bindgen::from_value(v.clone())?;
                        let v = self.val.right_prod(v.into());
                        let v = <$vec>::from(v);
                        res.push(v);
                    }
                    let res = res.into_boxed_slice();
                    let res = JsValue::from_serde(&res).unwrap();
                    Ok(res)
                }
            }

            impl Matrix {
                pub fn swap(&mut self, other: &mut Matrix) {
                    swap(self, other);
                }

                pub fn at(&self, col: usize, row: usize) -> $val {
                    // column major
                    self.values[row * $dim + col]
                }

                pub fn at_mut(&mut self, col: usize, row: usize) -> &mut $val {
                    &mut self.values[row * $dim + col]
                }

                pub fn left_prod(&self, v: [$val; $dim]) -> [$val; $dim] {
                    let mut res = [<$val>::default(); $dim];
                    for c in 0..$dim {
                        for r in 0..$dim {
                            res[c] += v[c] * self.at(c, r);
                        }
                    }
                    res
                }

                pub fn right_prod(&self, v: [$val; $dim]) -> [$val; $dim] {
                    let mut res = [<$val>::default(); $dim];
                    for c in 0..$dim {
                        for r in 0..$dim {
                            res[c] += v[c] * self.at(r, c);
                        }
                    }
                    res
                }
            }
        }
    };
}

impl_square_mat!(mat2i, i32, 2, Point2, Matrix2Int);
impl_square_mat!(mat2f, f32, 2, Vec2, Matrix2Float);
