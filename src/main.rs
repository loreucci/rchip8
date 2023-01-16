use std::env;
use std::fs;
use std::process;
use std::time::{Duration, Instant};

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
