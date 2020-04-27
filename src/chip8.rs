use sdl2::{event, keyboard::Keycode, pixels};
use std::collections;
use std::convert::TryInto;

const CHIP8_SCREEN_WIDTH: usize = 64;
const CHIP8_SCREEN_HEIGHT: usize = 32;
const NUM_PIXELS: usize = CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Chip8 {
    ram: [u8; 0x1000],
    pc: usize,
    ar: u16, // Address register
    v: [u16; 16],
    dt: u8,             // Delay timer
    st: u8,             // Sound timer
    stack: [usize; 48], // Stack implemented as empty ascending
    sp: usize,
    key_bindings: collections::HashMap<Keycode, usize>,
    key_pad: [bool; 16],
}

impl Chip8 {
    pub fn new(program: Vec<u8>) -> Chip8 {
        let mut chip8 = Chip8 {
            ram: [0; 0x1000],
            pc: 512,
            ar: 0,
            v: [0; 16],
            dt: 0,
            st: 0,
            stack: [0; 48],
            sp: 0,
            key_bindings: collections::HashMap::new(),
            key_pad: [false; 16],
        };

        let (reserved, ram) = chip8.ram.split_at_mut(512);
        assert!(reserved.len() == 512);
        let (ram_l, _ram_r) = ram.split_at_mut(program.len());
        ram_l.copy_from_slice(program.as_slice());

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

        // Insert font data
        let (font_area, _ram) = chip8.ram.split_at_mut(80);
        assert!(font_area.len() == 80);
        font_area.copy_from_slice(&FONT);

        return chip8;
    }

    pub fn run(&mut self) {
        let window_width: u32 = 1280;
        let window_height: u32 = 640;

        let sdl2_context = sdl2::init().expect("Failed to initialize SDL");
        let mut sdl2_timer_system = sdl2_context.timer().unwrap();
        let sdl2_video_system = sdl2_context.video().unwrap();
        // TODO: Perform scaling of 64x32 CHIP-8 Screen
        let window = sdl2_video_system
            .window("Chip8", window_width, window_height)
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        println!(
            "SDL2 version: {}.{}.{}",
            sdl2::version::version(),
            sdl2::version::revision(),
            sdl2::version::revision_number()
        );

        let mut pixels: [u8; NUM_PIXELS] = [0; NUM_PIXELS];
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                pixels::PixelFormatEnum::RGB24,
                CHIP8_SCREEN_WIDTH as u32,
                CHIP8_SCREEN_HEIGHT as u32,
            )
            .unwrap();

        let mut event_pump = sdl2_context.event_pump().unwrap();
        // TODO: Implement need to redraw (look at the Disp instructions)
        let mut redraw = true;
        let mut block = false;
        let mut key_pressed = false;
        let mut key = Keycode::A;
        let mut time = sdl2_timer_system.ticks();
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
                            key_pressed = true;
                            key = code;
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

            let first_half: u8 = self.ram[self.pc];
            let second_half: u8 = self.ram[self.pc + 1];
            let instruction: [u8; 4] = [
                (first_half & 0xF0) >> 4,
                first_half & 0xF,
                (second_half & 0xF0) >> 4,
                second_half & 0xF,
            ];
            #[cfg(debug_assertions)]
            {
                println!(
                    "Opcode: {:01X}{:01X}{:01X}{:01X}",
                    instruction[0], instruction[1], instruction[2], instruction[3]
                );
            }

            match instruction {
                // 00E0 - Clears the screen.
                [0x0, 0x0, 0xE, 0x0] => {
                    pixels = [0; NUM_PIXELS];
                    redraw = true;
                }
                // 00EE - Returns from a subroutine.
                [0x0, 0x0, 0xE, 0xE] => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp];
                }
                // 0NNN - Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
                // See issue.
                [0x0, _, _, _] => {
                    unimplemented!();
                }
                // 1NNN - Jumps to address NNN.
                [0x1, a, b, c] => {
                    let addr: usize = ((a as usize) << 8) | ((b as usize) << 4) | (c as usize);
                    self.pc = addr - 2;
                }
                // 2NNN - Calls subroutine at NNN.
                [0x2, a, b, c] => {
                    let addr: u16 = ((a as u16) << 8) | ((b as u16) << 4) | (c as u16);
                    self.stack[self.sp] = self.pc + 2;
                    self.sp += 1;
                    self.pc = (addr - 2) as usize;
                }
                // 3XNN - Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
                [0x3, x, b, c] => {
                    let val: u16 = ((b << 4) | c) as u16;
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
                    let nn = ((b << 4) | c) as u32;
                    let s: u32 = (self.v[x as usize] as u32) + nn;
                    self.v[x as usize] = (s % 0x10000) as u16;
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
                    let s: u32 = (self.v[x as usize] as u32) + (self.v[y as usize] as u32);
                    if s < 0x10000 {
                        self.v[0xF] = 0;
                    } else {
                        self.v[0xF] = 1;
                    }
                    self.v[x as usize] = (s % 0x10000) as u16;
                }
                // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                [0x8, x, y, 0x5] => {
                    let mut s: i32 = self.v[x as usize] as i32 - self.v[y as usize] as i32;
                    if s > 0 {
                        self.v[0xF] = 1;
                        self.v[x as usize] = s as u16;
                    } else {
                        self.v[0xF] = 0;
                        s = -s;
                    }
                    self.v[x as usize] = s as u16;
                }
                // 8XY6 - Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                [0x8, x, _y, 0x6] => {
                    self.v[0xF] = self.v[x as usize] % 0x1;
                    self.v[x as usize] >>= 1;
                }
                // 8XY7 - Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                [0x8, x, y, 0x7] => {
                    let mut s: i32 = self.v[y as usize] as i32 - self.v[x as usize] as i32;
                    if s > 0 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                        s = -s;
                    }
                    self.v[x as usize] = s as u16;
                }
                // 8XYE - Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                [0x8, x, _y, 0xE] => {
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
                    let addr: u16 = ((a as u16) << 8) | ((b as u16) << 4) | (c as u16);
                    self.ar = addr;
                }
                // BNNN - Jumps to the address NNN plus V0.
                [0xB, a, b, c] => {
                    let mut addr: usize = ((a as usize) << 8) | ((b as usize) << 4) | (c as usize);
                    addr += self.v[0] as usize;
                    self.pc = addr;
                }
                // CXNN - Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                [0xC, x, b, c] => {
                    let mut val: u16 = ((b << 4) | c) as u16;
                    val &= rand::random::<u8>() as u16;
                    self.v[x as usize] = val;
                }
                // DXYN - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
                // - Coordinate (VX, VY)                            - Check
                // - 8x{1-F} sprite, starts at I                    - Check
                // - Each row bit coded                             - Check
                // - I does not change                              - Check
                // - Flip from set to unset => VF=1, otherwise VF=0 - Check
                [0xD, x, y, c] => {
                    self.v[0xF] = 0;
                    let mut ar = self.ar as usize;
                    let vx: u16 = self.v[x as usize];
                    let vy: u16 = self.v[y as usize];
                    let mut yi = vy;
                    // Iterate over the rows
                    while yi < (vy + (c as u16)) {
                        if yi >= CHIP8_SCREEN_HEIGHT as u16 {
                            break;
                        }
                        // Extract each bit from sprite
                        let sprite_data: u8 = self.ram[ar];
                        let sprite_row = [
                            (sprite_data & 0x80) >> 7,
                            (sprite_data & 0x40) >> 6,
                            (sprite_data & 0x20) >> 5,
                            (sprite_data & 0x10) >> 4,
                            (sprite_data & 0x08) >> 3,
                            (sprite_data & 0x04) >> 2,
                            (sprite_data & 0x02) >> 1,
                            sprite_data & 0x01,
                        ];

                        // Get the current row in the screen buffer
                        let pixel_row_coord = (yi as usize) * CHIP8_SCREEN_WIDTH + (vx as usize);
                        let (pixel_row_left, mut pixel_row_right) =
                            (pixel_row_coord, pixel_row_coord + 8);
                        if pixel_row_left >= NUM_PIXELS {
                            break;
                        }
                        if pixel_row_right > NUM_PIXELS {
                            pixel_row_right = NUM_PIXELS;
                        }
                        let pixel_row = &mut pixels[pixel_row_left..pixel_row_right];

                        // Iterate over sprite pixels
                        let mut xi: u16 = 0;
                        for sprite_pixel in sprite_row.iter() {
                            let pixel: u8 = pixel_row[xi as usize];
                            // Collision detection
                            if pixel == 1 && *sprite_pixel == 1 {
                                self.v[0xF] = 1;
                            }
                            // XOR the pixels from the screen buffer and the sprite
                            let result = pixel ^ *sprite_pixel;
                            pixel_row[xi as usize] = result;
                            xi += 1;
                            if xi >= pixel_row.len() as u16 {
                                break;
                            }
                        }
                        ar += 1;
                        yi += 1;
                    }
                    redraw = true;
                }
                // EX9E - Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
                [0xE, x, 0x9, 0xE] => {
                    let vx: u16 = self.v[x as usize];
                    let key_pressed: bool = self.key_pad[vx as usize];
                    if key_pressed {
                        self.pc += 2;
                    }
                }
                // EXA1 - Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)
                [0xE, x, 0xA, 0x1] => {
                    let vx: u16 = self.v[x as usize];
                    let key_pressed: bool = self.key_pad[vx as usize];
                    if !key_pressed {
                        self.pc += 2;
                    }
                }
                // FX07 - Sets VX to the value of the delay timer.
                [0xF, x, 0x0, 0x7] => {
                    self.v[x as usize] = self.dt as u16;
                }
                // FX0A - A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
                [0xF, x, 0x0, 0xA] => {
                    block = true;
                    if key_pressed {
                        match self.key_bindings.get(&key) {
                            Some(binding) => {
                                self.v[x as usize] = *binding as u16;
                                block = false;
                            }
                            _ => {}
                        }
                    }
                }
                // FX15 - Sets the delay timer to VX.
                [0xF, x, 0x1, 0x5] => {
                    self.dt = self.v[x as usize] as u8;
                }
                // FX18 - Sets the sound timer to VX.
                [0xF, x, 0x1, 0x8] => {
                    self.st = self.v[x as usize] as u8;
                }
                // FX1E - Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't.
                [0xF, x, 0x1, 0xE] => {
                    self.ar += self.v[x as usize];
                    if self.ar > 0xFFF {
                        self.v[0xF] = 1;
                        self.ar %= 0x1000;
                    } else {
                        self.v[0xF] = 0;
                    }
                }
                // FX29 - Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                [0xF, x, 0x2, 0x9] => {
                    self.ar = (self.v[x as usize] % 16) * 5;
                }
                // FX33 - Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
                [0xF, x, 0x3, 0x3] => {
                    self.ram[self.ar as usize] = ((self.v[x as usize] - self.v[x as usize] % 100)
                        / 100)
                        .try_into()
                        .unwrap();
                    self.ram[(self.ar + 1) as usize] =
                        ((self.v[x as usize] - self.v[x as usize] % 10) / 10)
                            .try_into()
                            .unwrap();
                    self.ram[(self.ar + 2) as usize] =
                        (self.v[x as usize] % 10).try_into().unwrap();
                }
                // FX55 - Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
                [0xF, x, 0x5, 0x5] => {
                    let mut ar = self.ar as usize;
                    let mut xi = 0;
                    while xi < (x + 1) {
                        let reg = self.v[xi as usize];
                        let bytes = reg.to_be_bytes();
                        self.ram[ar] = bytes[1];
                        self.ram[ar + 1] = bytes[0];
                        ar += 2;
                        xi += 1;
                    }
                }
                // FX65 - Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
                [0xF, x, 0x6, 0x5] => {
                    let mut ar = self.ar as usize;
                    let mut xi = 0;
                    while xi < (x + 1) {
                        let reg = &mut self.v[xi as usize];
                        *reg = u16::from_be_bytes([self.ram[ar], self.ram[ar + 1]]);
                        ar += 2;
                        xi += 1;
                    }
                }
                [_, _, _, _] => {
                    panic!("Unknown instruction!");
                }
            }

            if block {
                block = false;
            } else {
                self.pc += 2;
            }

            if self.pc >= self.ram.len() {
                break;
            }

            if redraw {
                canvas.clear();

                let mut texture_data: [u8; NUM_PIXELS * 3] = [0; NUM_PIXELS * 3];
                for (i, pixel) in pixels.iter().enumerate() {
                    let mut color = 0x00;
                    if *pixel == 1 {
                        color = 0xFF;
                    }
                    texture_data[i * 3] = color;
                    texture_data[i * 3 + 1] = color;
                    texture_data[i * 3 + 2] = color;
                }
                texture
                    .update(None, &texture_data, (CHIP8_SCREEN_WIDTH * 3) as usize)
                    .unwrap();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();

                redraw = false;
            }

            sdl2_timer_system.delay(1);

            let end = sdl2_timer_system.ticks() - time;
            if end >= 16 {
                if self.dt > 0 {
                    self.dt -= 1;
                }
                if self.st > 0 {
                    self.st -= 1;
                }
                time = sdl2_timer_system.ticks();
            }

            #[cfg(debug_assertions)]
            {
                if instruction[0] != 6 {
                    //for (i, reg) in self.v.iter().enumerate() {
                    //    println!("V{:X}: {:X}", i, reg);
                    //}
                    //println!("ar: {:X}", self.ar);
                    //println!("pc: {:X}", self.pc);
                    //println!("sp: {:X}", self.sp);
                    //println!("dt: {:X}", self.dt);
                    //println!("st: {:X}", self.st);

                    //let mut line = String::new();
                    //std::io::stdin().read_line(&mut line).unwrap();
                }
            }
        }
    }
}
