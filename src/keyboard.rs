use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Keyboard {
    event_pump: EventPump,
    pub quit_requested: bool,
}

impl Keyboard {
    pub fn new(sdl: &Sdl) -> Result<Keyboard, String> {
        let event_pump = sdl.event_pump()?;
        Ok(Keyboard {
            event_pump,
            quit_requested: false,
        })
    }

    pub fn poll_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit_requested = true,
                _ => (),
            }
        }
    }
}
