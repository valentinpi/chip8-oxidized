mod schip8;

use schip8::SChip8;
use sdl2::{audio, event, keyboard::Keycode, pixels};
use std::{collections::HashMap, env, fs, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: chip8-oxidized <file-path>");
        return Err(io::Error::new(io::ErrorKind::Other, "Other"));
    }

    let error_message = format!("Unable to open {}", args[1]);
    let file: Vec<u8> = fs::read(&args[1]).expect(error_message.as_str());
    println!("{} is {} byte long", &args[1], file.len());

    let mut schip8 = SChip8::new(file.clone());

    let sdl2_context = sdl2::init().expect("Failed to initialize SDL");
    let sdl2_audio_system = sdl2_context.audio().unwrap();
    let mut sdl2_timer_system = sdl2_context.timer().unwrap();
    let sdl2_video_system = sdl2_context.video().unwrap();

    println!(
        "SDL2 version: {}.{}.{}",
        sdl2::version::version(),
        sdl2::version::revision(),
        sdl2::version::revision_number()
    );

    let key_bindings: HashMap<Keycode, usize> = [
        (Keycode::Num0, 0x0),
        (Keycode::Num1, 0x1),
        (Keycode::Num2, 0x2),
        (Keycode::Num3, 0x3),
        (Keycode::Num4, 0x4),
        (Keycode::Num5, 0x5),
        (Keycode::Num6, 0x6),
        (Keycode::Num7, 0x7),
        (Keycode::Num8, 0x8),
        (Keycode::Num9, 0x9),
        (Keycode::Kp0, 0x0),
        (Keycode::Kp1, 0x1),
        (Keycode::Kp2, 0x2),
        (Keycode::Kp3, 0x3),
        (Keycode::Kp4, 0x4),
        (Keycode::Kp5, 0x5),
        (Keycode::Kp6, 0x6),
        (Keycode::Kp7, 0x7),
        (Keycode::Kp8, 0x8),
        (Keycode::Kp9, 0x9),
        (Keycode::A, 0xA),
        (Keycode::B, 0xB),
        (Keycode::C, 0xC),
        (Keycode::D, 0xD),
        (Keycode::E, 0xE),
        (Keycode::F, 0xF)
    ].iter().cloned().collect();

    // TODO:
    let spec = audio::AudioSpecDesired {
        channels: Some(1),
        freq: Some(44100),
        samples: None,
    };
    let audio_device = sdl2_audio_system
        .open_playback(None, &spec, |spec| {
            struct SquareWave {
                phase_inc: f32,
                phase: f32,
                volume: f32,
            };

            impl audio::AudioCallback for SquareWave {
                // Data channel
                type Channel = f32;

                fn callback(&mut self, out: &mut [f32]) {
                    // Generate square wave
                    for x in out.iter_mut() {
                        if self.phase <= 0.5 {
                            *x = self.volume;
                        } else {
                            *x = -self.volume;
                        };
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                    }
                }
            }

            return SquareWave {
                phase: 0.0,
                phase_inc: 440.0 / spec.freq as f32,
                volume: 0.10,
            };
        })
        .unwrap();

    let window_width: u32 = 1280;
    let window_height: u32 = 640;

    let window = sdl2_video_system
        .window(
            ["chip8-oxidized", &args[1]].join(" - ").as_str(),
            window_width,
            window_height,
        )
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    let mut redraw = true;
    let mut key = 0;
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
                        key = 0;
                        match key_bindings.get(&code) {
                            Some(binding) => {
                                schip8.key_pad[*binding] = true;
                                key = *binding;
                            }
                            None => {}
                        }
                    }
                }
                KeyUp { keycode, .. } => {
                    if keycode != None {
                        let code = keycode.unwrap();
                        match key_bindings.get(&code) {
                            Some(binding) => schip8.key_pad[*binding] = false,
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if !schip8.run(key, &mut redraw) {
            break;
        }

        let end = sdl2_timer_system.ticks() - time;
        if end >= 16 {
            if schip8.dt > 0 {
                schip8.dt -= 1;
            }
            if schip8.st > 0 {
                schip8.st -= 1;
                audio_device.resume();
                if schip8.st == 0 {
                    audio_device.pause();
                }
            }
            time = sdl2_timer_system.ticks();
        }

        if redraw {
            canvas.clear();

            let mut texture = texture_creator
                .create_texture_streaming(
                    pixels::PixelFormatEnum::RGB24,
                    schip8.screen_width as u32,
                    schip8.screen_height as u32,
                )
                .unwrap();
            let num_pixels = schip8.screen_width * schip8.screen_height;
            let mut texture_data: Vec<u8> = vec![0; num_pixels * 3];
            for i in 0..num_pixels {
                let mut color = 0x00;
                let pixel = schip8.screen[i];

                if pixel == 1 {
                    color = 0xFF;
                }
                texture_data[i * 3] = color;
                texture_data[i * 3 + 1] = color;
                texture_data[i * 3 + 2] = color;
            }
            texture
                .update(None, &texture_data, (schip8.screen_width * 3) as usize)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            redraw = false;
        }
    }

    return Ok(());
}
