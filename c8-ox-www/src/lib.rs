use web_sys::console;
use wasm_bindgen::prelude::*;

//const tetris: &[u8] = include_bytes!("../../roms/chip8/TETRIS");

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    let js_value = JsValue::from_str("Hi!");
    let js_array = js_sys::Array::from(&js_value); 
    console::log(&js_array);
}
