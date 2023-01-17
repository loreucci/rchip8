use std::env;
use std::fs;
use std::process;
use std::time::{Duration, Instant};

use rand::Rng;

extern crate sdl2;

mod rchip8;
use rchip8::audio::Audio;
use rchip8::commons::CanTick;
use rchip8::display::Display;
use rchip8::keyboard::Keyboard;
use rchip8::memory;
use rchip8::timer::Timer;

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
    let mut args = env::args();
    args.next();
    let rom_path = match args.next() {
        Some(name) => name,
        None => print_error_and_quit("Not enough arguments, specify a path to a ROM."),
    };

    // memory, registers and stack
    let mut memory = [0u8; 4096];
    let mut v = [0u8; 16];
    let mut i = 0u16;
    let mut stack: Vec<u16> = Vec::new();
    let mut pc = 512u16;
    memory::load_rom(&mut memory, &rom_path).unwrap_or_else(|err| print_error_and_quit(&err));
    memory::load_character_set(&mut memory);

    // initialize sdl
    let sdl_context = sdl2::init().unwrap_or_else(|err| print_error_and_quit(&err));

    // create display and show it
    let mut display = Display::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));
    display.tick();

    // create keyboard manager
    let mut keyboard = Keyboard::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));

    // create audio device
    let mut audio = Audio::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));

    // timer
    let mut timer = Timer { time: 0 };

    // main loop
    'running: loop {
        let start_time = Instant::now();

        // fetch
        let opcode = ((memory[pc as usize] as u16) << 8) + memory[(pc + 1) as usize] as u16;
        // decode and execute
        match opcode & 0xF000 {
            0x1000 => {
                pc = get_nnn(opcode);
            }
            0x3000 => {
                if v[get_reg(opcode, 1)] == get_nn(opcode) {
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
            0xA000 => {
                i = opcode & 0xFFF;
                pc += 2;
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
            _ => print_error_and_quit(&format!(
                "Error: instruction {:#X} not implemented!",
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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60) - elapsed);
    }
}
