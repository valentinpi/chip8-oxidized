use std::{env, fs, io};
use sdl2::{
    audio,
    event,
    render,
    video
};

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
    pub fn run(&mut self) {
        loop {
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
                [0x0, 0x0, 0xE, 0x0] => {}
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
                [0x8, x, y, 0x0] => {}
                [0x8, x, y, 0x1] => {}
                [0x8, x, y, 0x2] => {}
                [0x8, x, y, 0x3] => {}
                [0x8, x, y, 0x4] => {}
                [0x8, x, y, 0x5] => {}
                [0x8, x, y, 0x6] => {}
                [0x8, x, y, 0x7] => {}
                [0x8, x, y, 0xE] => {}
                [0x9, x, y, 0x0] => {}
                [0xA, a, b, c] => {}
                [0xB, a, b, c] => {}
                [0xC, x, b, c] => {}
                [0xD, x, y, c] => {}
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

    let mut chip8 = Chip8 {
        ram: [0; 0x1000],
        pc: 0,
        ar: 0,
        v: [0; 16],
        dt: 0,
        st: 0,
        program: file.clone(),
    };
    chip8.run();

    for reg in chip8.v.iter() {
        println!("{}", reg);
    }

    return Ok(());
}
