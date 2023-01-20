use std::process;
use std::time::{Duration, Instant};

use clap::Parser;
use rand::Rng;

extern crate sdl2;

extern crate rchip8;
use rchip8::audio::Audio;
use rchip8::commons::CanTick;
use rchip8::display::Display;
use rchip8::keyboard::Keyboard;
use rchip8::memory;
use rchip8::timer::Timer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// emulated CPU frequency
    #[arg(short, long, default_value_t = 500)]
    freq: u32,

    /// ROM to execute
    rom: String,
}

fn print_error_and_quit(s: &str) -> ! {
    eprintln!("{}", s);
    process::exit(1)
}

fn get_reg(opcode: u16, pos: u8) -> usize {
    let shift = 4 * (3 - pos);
    usize::from((opcode & (0x000F << shift)) >> shift)
}

fn get_n(opcode: u16) -> u8 {
    (opcode & 0xF) as u8
}

fn get_nn(opcode: u16) -> u8 {
    (opcode & 0xFF) as u8
}

fn get_nnn(opcode: u16) -> u16 {
    opcode & 0xFFF
}

fn main() {
    // parse arguments
    let args = Args::parse();

    // clock frequency
    let cycle_duration = Duration::new(0, 1_000_000_000u32 / args.freq);

    // memory, registers and stack
    let mut memory = [0u8; 4096];
    let mut v = [0u8; 16];
    let mut i = 0u16;
    let mut stack: Vec<u16> = Vec::new();
    let mut pc = 512u16;
    memory::load_rom(&mut memory, &args.rom).unwrap_or_else(|err| print_error_and_quit(&err));
    memory::load_character_set(&mut memory);

    // initialize sdl
    let sdl_context = sdl2::init().unwrap_or_else(|err| print_error_and_quit(&err));

    // create display and show it
    let mut display = Display::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));
    display.tick();

    // create keyboard manager
    let mut keyboard = Keyboard::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));

    // create audio device
    let mut audio =
        Audio::new(&sdl_context, args.freq).unwrap_or_else(|err| print_error_and_quit(&err));

    // timer
    let mut timer = Timer::new(args.freq);

    // main loop
    'running: loop {
        let start_time = Instant::now();

        // fetch
        let opcode = ((memory[pc as usize] as u16) << 8) + memory[(pc + 1) as usize] as u16;
        // decode and execute
        match opcode & 0xF000 {
            0x0000 => {
                if opcode == 0x00E0 {
                    display.clear();
                    pc += 2;
                } else if opcode == 0x00EE {
                    pc = stack.pop().unwrap();
                } else {
                    stack.push(pc + 2);
                    pc = get_nnn(opcode);
                }
            }
            0x1000 => {
                pc = get_nnn(opcode);
            }
            0x2000 => {
                stack.push(pc + 2);
                pc = get_nnn(opcode);
            }
            0x3000 => {
                if v[get_reg(opcode, 1)] == get_nn(opcode) {
                    pc += 4;
                } else {
                    pc += 2;
                }
            }
            0x4000 => {
                if v[get_reg(opcode, 1)] != get_nn(opcode) {
                    pc += 4;
                } else {
                    pc += 2;
                }
            }
            0x5000 => {
                if v[get_reg(opcode, 1)] == v[get_reg(opcode, 2)] {
                    pc += 4;
                } else {
                    pc += 2;
                }
            }
            0x6000 => {
                v[get_reg(opcode, 1)] = get_nn(opcode);
                pc += 2;
            }
            0x7000 => {
                v[get_reg(opcode, 1)] += get_nn(opcode);
                pc += 2;
            }
            0x8000 => {
                let x = get_reg(opcode, 1);
                let y = get_reg(opcode, 2);
                match opcode & 0x000F {
                    0x0000 => {
                        v[x] = v[y];
                        pc += 2;
                    }
                    0x0001 => {
                        v[x] |= v[y];
                        pc += 2;
                    }
                    0x0002 => {
                        v[x] &= v[y];
                        pc += 2;
                    }
                    0x0003 => {
                        v[x] ^= v[y];
                        pc += 2;
                    }
                    0x0004 => {
                        let (s, c) = v[x].overflowing_add(v[y]);
                        v[x] = s;
                        if c {
                            v[0xF] = 1;
                        } else {
                            v[0xF] = 0;
                        }
                        pc += 2;
                    }
                    0x0005 => {
                        let b = if v[y] > v[x] { 0 } else { 1 };
                        v[x] -= v[y];
                        v[0xF] = b;
                        pc += 2;
                    }
                    0x0006 => {
                        v[0xF] = v[x] & 0x01;
                        v[x] >>= 1;
                        pc += 2;
                    }
                    0x0007 => {
                        let b = if v[x] > v[y] { 0 } else { 1 };
                        v[x] = v[y] - v[x];
                        v[0xF] = b;
                        pc += 2;
                    }
                    0x000E => {
                        v[0xF] = v[x] & 0x80;
                        v[x] <<= 1;
                        pc += 2;
                    }
                    _ => print_error_and_quit(&format!(
                        "Error: instruction {:#06X} not implemented!",
                        opcode
                    )),
                }
            }
            0x9000 => {
                if v[get_reg(opcode, 1)] != v[get_reg(opcode, 2)] {
                    pc += 4;
                } else {
                    pc += 2;
                }
            }
            0xA000 => {
                i = opcode & 0xFFF;
                pc += 2;
            }
            0xB000 => {
                pc = v[0] as u16 + get_nnn(opcode);
            }
            0xC000 => {
                let r = rand::thread_rng().gen_range(0..=255);
                v[get_reg(opcode, 1)] = r & get_nn(opcode);
                pc += 2;
            }
            0xD000 => {
                let x = get_reg(opcode, 1);
                let y = get_reg(opcode, 2);
                let n = get_n(opcode) as usize;
                let mut sprite = vec![0u8; n];
                for j in 0..n {
                    sprite[j] = memory[i as usize + j];
                }
                if display.draw(v[x], v[y], &sprite) {
                    v[0xF] = 1;
                } else {
                    v[0xF] = 0;
                }
                pc += 2;
            }
            0xE000 => {
                let x = get_reg(opcode, 1);
                match opcode & 0x00FF {
                    0x009E => {
                        if keyboard.is_down(v[x]) {
                            pc += 4;
                        } else {
                            pc += 2;
                        }
                    }
                    0x00A1 => {
                        if !keyboard.is_down(v[x]) {
                            pc += 4;
                        } else {
                            pc += 2;
                        }
                    }
                    _ => print_error_and_quit(&format!(
                        "Error: instruction {:#06X} not implemented!",
                        opcode
                    )),
                }
            }
            0xF000 => {
                let x = get_reg(opcode, 1);
                match opcode & 0x00FF {
                    0x0007 => {
                        v[x] = timer.get();
                        pc += 2;
                    }
                    0x00A => {
                        for k in 0..16 {
                            if keyboard.is_down(k) {
                                v[x] = k;
                                pc += 2;
                                break;
                            }
                        }
                    }
                    0x0015 => {
                        timer.set(x as u8);
                        pc += 2;
                    }
                    0x0018 => {
                        audio.play_sound(v[x]);
                        pc += 2;
                    }
                    0x001E => {
                        i += v[x] as u16;
                        pc += 2;
                    }
                    0x0029 => {
                        i = v[x] as u16 * 0x5;
                        pc += 2;
                    }
                    0x0033 => {
                        memory[i as usize] = v[x] / 100;
                        memory[i as usize + 1] = (v[x] % 100) / 10;
                        memory[i as usize + 2] = v[x] % 10;
                        pc += 2;
                    }
                    0x0055 => {
                        for j in 0..=x {
                            memory[i as usize + j] = v[j];
                        }
                        pc += 2;
                    }
                    0x0065 => {
                        for j in 0..=x {
                            v[j] = memory[i as usize + j];
                        }
                        pc += 2;
                    }
                    _ => print_error_and_quit(&format!(
                        "Error: instruction {:#06X} not implemented!",
                        opcode
                    )),
                }
            }
            _ => print_error_and_quit(&format!(
                "Error: instruction {:#06X} not implemented!",
                opcode
            )),
        }

        // process input keys
        keyboard.tick();
        if keyboard.quit_requested {
            break 'running;
        }

        // update components
        timer.tick();
        audio.tick();
        display.tick();

        // sleep
        let elapsed = start_time.elapsed();
        if elapsed < cycle_duration {
            ::std::thread::sleep(cycle_duration - elapsed);
        }
    }
}
