use sdl2::{audio, event, render, video};
use std::{env, fs, io};

const CHIP8_SCREEN_WIDTH: usize = 64;
const CHIP8_SCREEN_HEIGHT: usize = 32;
const NUM_PIXELS: usize = CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT;
const NUM_PIXELS_BYTE: usize = CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT * 4;

struct Chip8 {
    ram: [u8; 0x1000],
    pc: usize,
    ar: u16, // Address register
    v: [u16; 16],
    dt: u8, // Delay timer
    st: u8, // Sound timer
    program: Vec<u8>,
}

impl Chip8 {
    pub fn new(program: Vec<u8>) -> Chip8 {
        let chip8 = Chip8 {
            ram: [0; 0x1000],
            pc: 0,
            ar: 0,
            v: [0; 16],
            dt: 0,
            st: 0,
            program,
        };

        return chip8;
    }

    pub fn run(&mut self) {
        let window_width: u32 = 1280;
        let window_height: u32 = 640;

        let sdl2_context = sdl2::init().expect("Failed to initialize SDL");
        let sdl2_videosystem = sdl2_context.video().unwrap();
        // TODO: Perform scaling of 64x32 CHIP-8 Screen
        let window = sdl2_videosystem
            .window("Chip8", window_width, window_height)
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        // Create texture and corresponding pixel array
        let mut pixels: [u8; NUM_PIXELS] = [0; NUM_PIXELS];
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGBA32,
                window_width,
                window_height,
            )
            .unwrap();

        let mut event_pump = sdl2_context.event_pump().unwrap();
        // TODO: Implement need to redraw (look at the Disp instructions)
        let mut redraw = false;
        'running: loop {
            for event in event_pump.poll_iter() {
                use event::Event::*;
                match event {
                    Quit { .. } => {
                        break 'running;
                    }
                    _ => {
                        println!("EVENT");
                    }
                }
            }

            let first_half = self.program[self.pc] as u8;
            let second_half = self.program[self.pc + 1] as u8;
            let instruction: [u8; 4] = [
                (first_half & 0xF0) >> 4,
                first_half & 0xF,
                (second_half & 0xF0) >> 4,
                second_half & 0xF,
            ];
            println!(
                "Opcode: {:01X}{:01X}{:01X}{:01X}",
                instruction[0], instruction[1], instruction[2], instruction[3]
            );

            match instruction {
                [0x0, 0x0, 0xE, 0x0] => {
                    let mut i = 0;
                    while i < NUM_PIXELS_BYTE {
                        pixels[i] = 0x00;
                        i += 1;
                    }
                    redraw = true;
                }
                [0x0, 0x0, 0xE, 0xE] => {}
                [0x0, a, b, c] => {}
                [0x1, a, b, c] => {}
                [0x2, a, b, c] => {}
                [0x3, a, b, c] => {}
                [0x4, a, b, c] => {}
                [0x5, a, b, c] => {}
                [0x6, a, b, c] => {
                    self.v[a as usize] = ((c << 4) | b) as u16;
                }
                [0x7, a, b, c] => {}
                [0x8, x, y, 0x0] => {
                    self.v[x as usize] = self.v[y as usize];
                }
                [0x8, x, y, 0x1] => {
                    self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                }
                [0x8, x, y, 0x2] => {
                    self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                }
                [0x8, x, y, 0x3] => {
                    self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                }
                [0x8, x, y, 0x4] => {
                    let s = self.v[x as usize] as u32 + self.v[y as usize] as u32;
                    if s < std::u16::MAX {
                        self.v[0xF] = 0;
                        self.v[x as usize] = s as u16;
                    } else {
                        self.v[0xF] = 1;
                        self.v[x as usize] = (s & 0xFFFF) as u16;
                    }
                }
                [0x8, x, y, 0x5] => {
                    let s: i32 = self.v[x as usize] as i32 - self.v[y as usize] as i32;
                    if s > 0 {
                        self.v[0xF] = 1;
                        self.v[x as usize] = s as u16;
                    } else {
                        self.v[0xF] = 0;
                        self.v[x as usize] = -s as u16;
                    }
                }
                [0x8, x, y, 0x6] => {}
                [0x8, x, y, 0x7] => {}
                [0x8, x, y, 0xE] => {}
                [0x9, x, y, 0x0] => {}
                [0xA, a, b, c] => {}
                [0xB, a, b, c] => {}
                [0xC, x, b, c] => {}
                [0xD, x, y, c] => {
                    // TODO: Implement this efficiently
                    // Watched this naming convention in a tutorial once
                    //let sprite_width = 8;
                    //let mut yy = y;
                    //while yy < (y + c) {
                    //    pixels[(yy * sprite_width) as usize] = self.ram[self.ar as usize..(self.ar + 8) as usize];
                    //    yy += 1;
                    //}
                }
                [0xE, x, 0x9, 0xE] => {}
                [0xE, x, 0xA, 0x1] => {}
                [0xF, x, 0x0, 0x7] => {}
                [0xF, x, 0x0, 0xA] => {}
                [0xF, x, 0x1, 0x5] => {}
                [0xF, x, 0x1, 0x8] => {}
                [0xF, x, 0x1, 0xE] => {}
                [0xF, x, 0x2, 0x9] => {}
                [0xF, x, 0x3, 0x3] => {}
                [0xF, x, 0x5, 0x5] => {}
                [0xF, x, 0x6, 0x5] => {}
                [_, _, _, _] => {
                    panic!("Unknown instruction!");
                }
            }

            self.pc += 2;
            if self.pc >= self.program.len() {
                break;
            }

            if redraw {
                texture
                    .update(None, pixels.as_ref(), (64 * 4) as usize)
                    .unwrap();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: chip8-oxidized <file-path>");
        return Err(io::Error::new(io::ErrorKind::Other, "Other"));
    }

    let error_message = format!("Unable to open {}", args[1]);
    let file: Vec<u8> = fs::read(&args[1]).expect(error_message.as_str());
    println!("{} is {} byte long", &args[1], file.len());

    let mut chip8 = Chip8::new(file.clone());
    chip8.run();

    for reg in chip8.v.iter() {
        println!("{}", reg);
    }

    return Ok(());
}
