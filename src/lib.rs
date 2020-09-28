pub mod hex;
pub mod mat;
pub mod array;
pub mod vec;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn debug(s: &str);
}

/// Initialize the global state of the library.
///
/// Users should not have to call this method.
#[wasm_bindgen(start)]
pub fn _start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    debug("CaoMath initialized successfully");
}
