//! Hex-Grid utilities
use crate::mat::mat2::Mat2f;
use crate::array::Array3f;
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use wasm_bindgen::prelude::*;

const SQRT3APROX: f32 = 1.732_050_807_57;

/// Converts a point on the hex grid from `cube` representation to `axial` representation
#[wasm_bindgen(js_name = cubeToAxial)]
pub fn cube_to_axial(cube: &Vec3) -> Vec2 {
    let q = cube.x;
    let r = cube.z;
    [q, r].into()
}

/// Converts a point on the hex grid from `axial` representation to `cube` representation
#[wasm_bindgen(js_name = axialToCube)]
pub fn axial_to_cube(axial: &Vec2) -> Vec3 {
    let x = axial.x;
    let z = axial.y;
    let y = -x - z;
    [x, y, z].into()
}

/// 'Manhatten-esque' distance between two 'cube' points on the hex grid
#[wasm_bindgen(js_name = cubeDistance)]
pub fn cube_distance(a: &Vec3, b: &Vec3) -> u32 {
    let x = (a.x - b.x).abs();
    let y = (a.y - b.y).abs();
    let z = (a.z - b.z).abs();
    x.max(y).max(z) as u32
}

/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "pointy top grid".
#[wasm_bindgen(js_name = axialToPixelMatrixPointy)]
pub fn axial_to_pixel_mat_pointy() -> Mat2f {
    let mat: Mat2f = [[SQRT3APROX, 0.], [SQRT3APROX / 2.0, 3. / 2.]].into();
    mat.into()
}

/// Calculate the pixel position of the hex point represented by axial coordinates by using
/// `rightProd`. Assumes "flat top grid".
#[wasm_bindgen(js_name = axialToPixelMatrixFlat)]
pub fn axial_to_pixel_mat_flat() -> Mat2f {
    let mat: Mat2f = [[3. / 2., SQRT3APROX / 2.], [0., SQRT3APROX]].into();
    mat.into()
}

/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "pointy top grid".
#[wasm_bindgen(js_name = pixelToAxialMatrixPointy)]
pub fn pixel_to_axial_pointy() -> Mat2f {
    let mat: Mat2f = [[SQRT3APROX / 3., 0.], [-1. / 3., 2. / 3.]].into();
    mat.into()
}

/// Calculate the axial hex grid position of the pixel by using
/// `rightProd`. Assumes "flat top grid".
#[wasm_bindgen(js_name = pixelToAxialMatrixFlat)]
pub fn pixel_to_axial_flat() -> Mat2f {
    let mat: Mat2f = [[2. / 3., -1. / 3.], [0., SQRT3APROX / 3.]].into();
    mat.into()
}

#[wasm_bindgen(js_name = roundToNearestAxial)]
pub fn round_to_nearest_axial(axial: &Vec2) -> Vec2 {
    // convert to cube
    let [x, z]: [f32; 2] = axial.into();
    let y = -x - z;
    // round
    let [mut rx, ry, mut rz] = [x.round(), y.round(), z.round()];
    let [dx, dy, dz] = [(rx - x).abs(), (ry - y).abs(), (rz - z).abs()];
    if dx > dy && dx > dz {
        rx = -ry - rz;
    } else if dy <= dz {
        rz = -rx - ry;
    }
    // convert back to axial
    Vec2::new(rx, rz)
}

/// Return a list of points, each point will be inside a hex that is intersected by the segment
/// between points `a` and `b`, skipping `a` and `b`.
#[wasm_bindgen(js_name=cubeSegmentPoints)]
pub fn cube_segment_points(a: &Vec3, b: &Vec3) -> Array3f {
    let n = cube_distance(a, b);
    let np1 = 1. / (n as f32);
    let data = (1..n).map(|i| cube_lerp(a, b, np1 * i as f32)).collect();

    Array3f { data }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn cube_lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    [lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t)].into()
}
