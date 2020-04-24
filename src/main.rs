use sdl2::{audio, event, keyboard::Keycode, render, video};
use std::{collections, env, fs, io};

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
    key_bindings: collections::HashMap<Keycode, usize>,
    key_pad: [bool; 16],
}

impl Chip8 {
    pub fn new(program: Vec<u8>) -> Chip8 {
        let mut chip8 = Chip8 {
            ram: [0; 0x1000],
            pc: 0,
            ar: 0,
            v: [0; 16],
            dt: 0,
            st: 0,
            program,
            key_bindings: collections::HashMap::new(),
            key_pad: [false; 16],
        };

        chip8.key_bindings.insert(Keycode::Num0, 0x0);
        chip8.key_bindings.insert(Keycode::Num1, 0x1);
        chip8.key_bindings.insert(Keycode::Num2, 0x2);
        chip8.key_bindings.insert(Keycode::Num3, 0x3);
        chip8.key_bindings.insert(Keycode::Num4, 0x4);
        chip8.key_bindings.insert(Keycode::Num5, 0x5);
        chip8.key_bindings.insert(Keycode::Num6, 0x6);
        chip8.key_bindings.insert(Keycode::Num7, 0x7);
        chip8.key_bindings.insert(Keycode::Num8, 0x8);
        chip8.key_bindings.insert(Keycode::Num9, 0x9);
        chip8.key_bindings.insert(Keycode::A, 0xA);
        chip8.key_bindings.insert(Keycode::B, 0xB);
        chip8.key_bindings.insert(Keycode::C, 0xC);
        chip8.key_bindings.insert(Keycode::D, 0xD);
        chip8.key_bindings.insert(Keycode::E, 0xE);
        chip8.key_bindings.insert(Keycode::F, 0xF);

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
                    KeyDown { keycode, .. } => {
                        if keycode != None {
                            let code = keycode.unwrap();
                            match self.key_bindings.get(&code) {
                                Some(binding) => self.key_pad[*binding] = true,
                                None => {}
                            }
                        }
                    }
                    KeyUp { keycode, .. } => {
                        if keycode != None {
                            let code = keycode.unwrap();
                            match self.key_bindings.get(&code) {
                                Some(binding) => self.key_pad[*binding] = false,
                                None => {}
                            }
                        }
                    }
                    _ => {}
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
                // 00E0 - Clears the screen.
                [0x0, 0x0, 0xE, 0x0] => {
                    let mut i = 0;
                    while i < NUM_PIXELS_BYTE {
                        pixels[i] = 0x00;
                        i += 1;
                    }
                    redraw = true;
                }
                // 00EE - Returns from a subroutine.
                [0x0, 0x0, 0xE, 0xE] => {}
                // 0NNN - Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
                [0x0, a, b, c] => {}
                // 1NNN - Jumps to address NNN.
                [0x1, a, b, c] => {
                    let mut addr: usize = 0;
                    addr |= (a as usize) << 8;
                    addr |= (b as usize) << 4;
                    addr |= c as usize;
                    self.pc = addr - 2;
                }
                // 2NNN - Calls subroutine at NNN.
                [0x2, a, b, c] => {}
                // 3XNN - Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
                [0x3, x, b, c] => {
                    let mut val: u16 = 0;
                    val |= (b as u16) << 4;
                    val |= c as u16;
                    if self.v[x as usize] == val {
                        self.pc += 2;
                    }
                }
                // 4XNN - Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)
                [0x4, x, b, c] => {
                    let val = ((b << 4) | c) as u16;
                    if self.v[x as usize] != val {
                        self.pc += 2;
                    }
                }
                // 5XNN - Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
                [0x5, x, y, 0] => {
                    if self.v[x as usize] == self.v[y as usize] {
                        self.pc += 2;
                    }
                }
                // 6XNN - Sets VX to NN.
                [0x6, x, b, c] => {
                    self.v[x as usize] = ((b << 4) | c) as u16;
                }
                // 7XNN - Adds NN to VX. (Carry flag is not changed)
                [0x7, x, b, c] => {
                    let val = ((b << 4) | c) as u16;
                    self.v[x as usize] += val;
                }
                // 8XY0 - Sets VX to the value of VY.
                [0x8, x, y, 0x0] => {
                    self.v[x as usize] = self.v[y as usize];
                }
                // 8XY1 - Sets VX to VX or VY. (Bitwise OR operation)
                [0x8, x, y, 0x1] => {
                    self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                }
                // 8XY2 - Sets VX to VX and VY. (Bitwise AND operation)
                [0x8, x, y, 0x2] => {
                    self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                }
                // 8XY3 - Sets VX to VX xor VY.
                [0x8, x, y, 0x3] => {
                    self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                }
                // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                [0x8, x, y, 0x4] => {
                    let s = self.v[x as usize] as u32 + self.v[y as usize] as u32;
                    if s < std::u16::MAX as u32 {
                        self.v[0xF] = 0;
                        self.v[x as usize] = s as u16;
                    } else {
                        self.v[0xF] = 1;
                        self.v[x as usize] = (s & 0xFFFF) as u16;
                    }
                }
                // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
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
                // 8XY6 - Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                [0x8, x, y, 0x6] => {
                    self.v[0xF] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;
                }
                // 8XY7 - Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                [0x8, x, y, 0x7] => {
                    let s: i32 = self.v[y as usize] as i32 - self.v[x as usize] as i32;
                    if s > 0 {
                        self.v[0xF] = 1;
                        self.v[x as usize] = s as u16;
                    } else {
                        self.v[0xF] = 0;
                        self.v[x as usize] = -s as u16;
                    }
                }
                // 8XYE - Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                [0x8, x, y, 0xE] => {
                    self.v[0xF] = self.v[x as usize] & 0x8000;
                    self.v[x as usize] <<= 1;
                }
                // 9XY0 - Skips the next instruction if VX doesn't equal VY.
                [0x9, x, y, 0x0] => {
                    if self.v[x as usize] != self.v[y as usize] {
                        self.pc += 2;
                    }
                }
                // ANNN - Sets I to the address NNN.
                [0xA, a, b, c] => {
                    let mut addr: u16 = 0;
                    addr |= (a as u16) << 8;
                    addr |= (b as u16) << 4;
                    addr |= c as u16;
                    self.ar = addr;
                }
                // BNNN - Jumps to the address NNN plus V0.
                [0xB, a, b, c] => {
                    let mut addr: usize = 0;
                    addr |= (a as usize) << 8;
                    addr |= (b as usize) << 4;
                    addr |= c as usize;
                    addr += self.v[0] as usize;
                    self.pc = addr;
                }
                // CXNN - Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                [0xC, x, b, c] => {
                    let mut addr: u16 = 0;
                    addr |= (b as u16) << 4;
                    addr |= c as u16;
                    addr &= rand::random::<u16>();
                    self.v[x as usize] = addr;
                }
                // DXYN - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
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
                // EX9E - Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
                [0xE, x, 0x9, 0xE] => {}
                // EXA1 - Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)
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
