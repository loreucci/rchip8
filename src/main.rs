use std::process;
use std::time::Duration;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod display;
use display::Display;

fn print_error_and_quit(s: &str) -> ! {
    eprintln!("{}", s);
    process::exit(1)
}

fn main() {
    // initialize sdl
    let sdl_context = sdl2::init().unwrap_or_else(|err| print_error_and_quit(&err));

    // create display and show it
    let mut display = Display::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));
    display.draw();

    // temporary input management (1)
    let mut event_pump = sdl_context
        .event_pump()
        .unwrap_or_else(|err| print_error_and_quit(&err));

    // main loop
    'running: loop {
        // temporary input management (2)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // update display
        display.draw();

        // sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
