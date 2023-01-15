use std::process;
use std::time::Duration;

extern crate sdl2;

mod rchip8;
use rchip8::commons::CanTick;
use rchip8::display::Display;
use rchip8::keyboard::Keyboard;

fn print_error_and_quit(s: &str) -> ! {
    eprintln!("{}", s);
    process::exit(1)
}

fn main() {
    // initialize sdl
    let sdl_context = sdl2::init().unwrap_or_else(|err| print_error_and_quit(&err));

    // create display and show it
    let mut display = Display::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));
    display.tick();

    // create keyboard manager
    let mut keyboard = Keyboard::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));

    // main loop
    'running: loop {
        // process input keys
        keyboard.tick();
        if keyboard.quit_requested {
            break 'running;
        }

        // update display
        display.tick();

        // sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
