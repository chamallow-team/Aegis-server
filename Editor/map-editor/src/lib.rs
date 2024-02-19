mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, map-editor!");
}

#[wasm_bindgen]
pub struct Map {
    map: map::Map
}
