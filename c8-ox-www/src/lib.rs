mod schip8;

use schip8::SChip8;
//use web_sys::console;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    //let mut arr = [1,2,3,4];
    //arr[1] = 2;
    //console::log_1(&JsValue::from_str("Hi!"));

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    let tetris = include_bytes!("../../roms/chip8/TETRIS").to_vec();
    let schip8 = SChip8::new(tetris);
}
