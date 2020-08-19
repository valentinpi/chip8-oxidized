mod schip8;

use schip8::SChip8;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let canvas = window
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let tetris = include_bytes!("../../roms/chip8/TETRIS").to_vec();
    let mut schip8 = SChip8::new(tetris);
    //loop {
        let quit = schip8.run(20);

        schip8.play();
        schip8.render(&canvas, &context);

        if !quit {
            //break;
        }
    //}
}
