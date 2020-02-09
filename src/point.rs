use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Index<usize> for Point2 {
    type Output = i32;
    fn index(&self, index: usize) -> &i32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point index {} is out of range", index),
        }
    }
}

impl IndexMut<usize> for Point2 {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point index {} is out of range", index),
        }
    }
}

impl AddAssign for Point2 {
    fn add_assign(&mut self, p: Self) {
        self.x += p.x;
        self.y += p.y;
    }
}

impl Add for Point2 {
    type Output = Self;

    fn add(mut self, p: Self) -> Self {
        self += p;
        self
    }
}

impl SubAssign for Point2 {
    fn sub_assign(&mut self, p: Self) {
        self.x -= p.x;
        self.y -= p.y;
    }
}

impl Sub for Point2 {
    type Output = Self;

    fn sub(mut self, p: Self) -> Self {
        self -= p;
        self
    }
}

impl MulAssign<i32> for Point2 {
    fn mul_assign(&mut self, a: i32) {
        self.x *= a;
        self.y *= a;
    }
}

impl Mul<i32> for Point2 {
    type Output = Self;

    fn mul(mut self, a: i32) -> Self {
        self *= a;
        self
    }
}
impl MulAssign<f32> for Point2 {
    fn mul_assign(&mut self, a: f32) {
        let Point2 { x, y } = self;
        let [mut x, mut y] = [*x as f32, *y as f32];
        x *= a;
        y *= a;
        self.x = x as i32;
        self.y = y as i32;
    }
}

impl Mul<f32> for Point2 {
    type Output = Self;

    fn mul(mut self, a: f32) -> Self {
        self *= a;
        self
    }
}
impl DivAssign<i32> for Point2 {
    fn div_assign(&mut self, a: i32) {
        self.x /= a;
        self.y /= a;
    }
}

impl Div<i32> for Point2 {
    type Output = Self;

    fn div(mut self, a: i32) -> Self {
        self /= a;
        self
    }
}
impl DivAssign<f32> for Point2 {
    fn div_assign(&mut self, a: f32) {
        let Point2 { x, y } = self;
        let [mut x, mut y] = [*x as f32, *y as f32];
        x /= a;
        y /= a;
        self.x = x as i32;
        self.y = y as i32;
    }
}

impl Div<f32> for Point2 {
    type Output = Self;

    fn div(mut self, a: f32) -> Self {
        self /= a;
        self
    }
}
