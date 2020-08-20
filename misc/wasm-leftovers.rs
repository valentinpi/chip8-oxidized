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
    loop {
        let quit = schip8.run(20);

        if !quit {
            break;
        }
    }
}


   // Since the screen array must be private, use this for rendering
    pub fn render(
        &self,
        canvas: &web_sys::HtmlCanvasElement,
        context: &web_sys::CanvasRenderingContext2d,
    ) {
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        let pixel_w = canvas.width() as usize / self.screen_width;
        let pixel_h = canvas.height() as usize / self.screen_height;

        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                let pixel = self.screen[y * SCHIP8_SCREEN_WIDTH + x];

                if pixel == 0 {
                    context.set_fill_style(&JsValue::from_str("black"));
                } else {
                    context.set_fill_style(&JsValue::from_str("white"));
                }

                context.fill_rect(
                    (x * pixel_w) as f64,
                    (y * pixel_h) as f64,
                    pixel_w as f64,
                    pixel_h as f64,
                );

                //log!("Running");
            }
        }
    }
