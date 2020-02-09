pub mod hex;
pub mod mat;
pub mod point;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub trait Descriptor {
    type ValueType;
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Init the error handling of the library
pub fn init_error_handling() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
