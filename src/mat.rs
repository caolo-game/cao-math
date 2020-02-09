//! Basic fix sized matrices
use crate::point::*;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use wasm_bindgen::prelude::*;

macro_rules! impl_square_mat {
    ($mat: ident, $val: ty, $dim: expr, $vec: ident, $proxy: ident) => {
        pub mod $mat {
            use super::*;
            use $vec::*;

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
                        let v: Point = serde_wasm_bindgen::from_value(v.clone())?;
                        let v = self.val.left_prod(v.into());
                        let v = <Point>::from(v);
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
                        let v: Point = serde_wasm_bindgen::from_value(v.clone())?;
                        let v = self.val.right_prod(v.into());
                        let v = <Point>::from(v);
                        res.push(v);
                    }
                    let res = res.into_boxed_slice();
                    let res = JsValue::from_serde(&res).unwrap();
                    Ok(res)
                }
            }

            impl Matrix {
                #[inline]
                pub fn swap(&mut self, other: &mut Matrix) {
                    swap(self, other);
                }

                #[inline]
                pub fn at(&self, col: usize, row: usize) -> $val {
                    assert!(col < $dim);
                    assert!(row < $dim);
                    self.values[row * $dim + col]
                }

                #[inline]
                pub fn at_mut(&mut self, col: usize, row: usize) -> &mut $val {
                    assert!(col < $dim);
                    assert!(row < $dim);
                    &mut self.values[row * $dim + col]
                }

                #[inline]
                pub fn left_prod(&self, v: [$val; $dim]) -> [$val; $dim] {
                    let mut res = [<$val>::default(); $dim];
                    for c in 0..$dim {
                        for r in 0..$dim {
                            res[c] += v[r] * self.at(c, r);
                        }
                    }
                    res
                }

                #[inline]
                pub fn right_prod(&self, v: [$val; $dim]) -> [$val; $dim] {
                    let mut res = [<$val>::default(); $dim];
                    for r in 0..$dim {
                        for c in 0..$dim {
                            res[r] += v[c] * self.at(c, r);
                        }
                    }
                    res
                }
            }
        }
    };
}

impl_square_mat!(mat2i32, i32, 2, vec2i32, Matrix2Int);
impl_square_mat!(mat2f32, f32, 2, vec2f32, Matrix2Float);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_right_prod_2by2() {
        use mat2f32::*;

        const SQRT3APROX: f32 = 1.73205080757;

        let mut mat = Matrix::default();
        *mat.at_mut(0, 0) = SQRT3APROX;
        *mat.at_mut(1, 0) = SQRT3APROX / 2.0;
        *mat.at_mut(0, 1) = 0.0;
        *mat.at_mut(1, 1) = 3.0 / 2.0;

        let p = [1., 2.];
        let res = mat.right_prod(p);

        assert_eq!(res, [SQRT3APROX * 2., 3.]);
    }

    #[test]
    fn basic_left_prod_2by2() {
        use mat2f32::*;

        const SQRT3APROX: f32 = 1.73205080757;

        let mut mat = Matrix::default();
        *mat.at_mut(0, 0) = SQRT3APROX;
        *mat.at_mut(1, 0) = SQRT3APROX / 2.0;
        *mat.at_mut(0, 1) = 0.0;
        *mat.at_mut(1, 1) = 3.0 / 2.0;

        let p = [1., 2.];
        let res = mat.left_prod(p);

        assert_eq!(res, [SQRT3APROX , 3.8660254]);
    }
}
