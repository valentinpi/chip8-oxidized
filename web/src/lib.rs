mod schip8;

use wasm_bindgen::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // From this example https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("chip8-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let tetris: Vec<u8> = include_bytes!("../../examples/chip8/TETRIS").into_iter().cloned().collect();

    let message_str = String::from("Hello web-sys!");
    let message_val = serde_wasm_bindgen::to_value(&message_str).unwrap();
    web_sys::console::log(&js_sys::Array::from(&message_val));

    canvas.fill_rect(0, 0, canvas.width, canvas.height);

    //let mut schip8 = schip8::SChip8::new(tetris);
    //let mut redraw = true;
    //loop {
    //    schip8.run(0, &mut redraw);
    //}
}
