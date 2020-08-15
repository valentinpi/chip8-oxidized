//use web_sys::console;
use wasm_bindgen::prelude::*;

//const tetris: &[u8] = include_bytes!("../examples/chip8/TETRIS");

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    //console.log("Hi!");
}
