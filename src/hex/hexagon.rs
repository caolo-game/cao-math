use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::vec::vec2::Vec2;

use super::axial_to_cube;

#[wasm_bindgen(js_name=Hexagon, inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Hexagon {
    /// center point in Vec2 coordinate system
    pub center: Vec2,
    pub radius: f32,
}

#[wasm_bindgen(js_class=Hexagon)]
impl Hexagon {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            center: Vec2::new(0.0, 0.0),
            radius: 0.0,
        }
    }

    pub fn from_radius(radius: f32) -> Self {
        Self {
            radius,
            center: Vec2::new(radius, radius),
        }
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        let point = *point - self.center;
        let crate::vec::vec3::Vec3 { x, y, z } = axial_to_cube(&point);
        let r = self.radius;
        x.abs() <= r && y.abs() <= r && z.abs() <= r
    }

    /// Generates a list of point in the Axial coordinate system that build up this hexagon grid.
    ///
    /// @return list of tuples of [q, r] integer coordinates.
    pub fn list_points(&self) -> Box<[JsValue]> {
        let radius = self.radius as i32;
        let Vec2 { x, y } = self.center;
        let center = [x as i32, y as i32];
        (-radius..=radius)
            .flat_map(move |x| {
                let fromy = (-radius).max(-x - radius);
                let toy = radius.min(-x + radius);
                (fromy..=toy)
                    .map(move |y| {
                        let y = -x - y;
                        [x + center[0], y + center[1]]
                    })
                    .map(|p| JsValue::from_serde(&p).unwrap())
            })
            .collect()
    }

    pub fn with_center(&mut self, center: &Vec2) -> Self {
        self.center = *center;
        *self
    }

    pub fn with_offset(&mut self, offset: &Vec2) -> Self {
        self.center += *offset;
        *self
    }

    pub fn with_radius(&mut self, radius: f32) -> Self {
        self.radius = radius;
        *self
    }
}
