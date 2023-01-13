use std::process;
use std::time::Duration;

extern crate sdl2;

mod display;
use display::Display;

mod keyboard;
use keyboard::Keyboard;

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

    // create keyboard manager
    let mut keyboard = Keyboard::new(&sdl_context).unwrap_or_else(|err| print_error_and_quit(&err));

    // main loop
    'running: loop {
        // process input keys
        keyboard.poll_events();
        if keyboard.quit_requested {
            break 'running;
        }

        // update display
        display.draw();

        // sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
