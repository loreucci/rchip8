use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::Sdl;

use super::commons::CanTick;

pub struct Keyboard {
    event_pump: EventPump,
    pub quit_requested: bool,
    keys: [bool; 16],
    key_map: HashMap<Keycode, usize>,
}

impl Keyboard {
    pub fn new(sdl: &Sdl) -> Result<Keyboard, String> {
        let event_pump = sdl.event_pump()?;

        // create key mapping
        let mut key_map: HashMap<Keycode, usize> = HashMap::new();
        key_map.insert(Keycode::X, 0x0);
        key_map.insert(Keycode::Num1, 0x1);
        key_map.insert(Keycode::Num2, 0x2);
        key_map.insert(Keycode::Num3, 0x3);
        key_map.insert(Keycode::Q, 0x4);
        key_map.insert(Keycode::W, 0x5);
        key_map.insert(Keycode::E, 0x6);
        key_map.insert(Keycode::A, 0x7);
        key_map.insert(Keycode::S, 0x8);
        key_map.insert(Keycode::D, 0x9);
        key_map.insert(Keycode::Z, 0xA);
        key_map.insert(Keycode::C, 0xB);
        key_map.insert(Keycode::Num4, 0xC);
        key_map.insert(Keycode::R, 0xD);
        key_map.insert(Keycode::F, 0xE);
        key_map.insert(Keycode::V, 0xF);

        Ok(Keyboard {
            event_pump,
            quit_requested: false,
            keys: [false; 16],
            key_map,
        })
    }

    pub fn is_down(&self, k: u8) -> bool {
        self.keys[k as usize]
    }
}

impl CanTick for Keyboard {
    fn tick(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit_requested = true,
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match self.key_map.get(&k) {
                    Some(i) => self.keys[*i] = true,
                    None => (),
                },
                Event::KeyUp {
                    keycode: Some(k), ..
                } => match self.key_map.get(&k) {
                    Some(i) => self.keys[*i] = false,
                    None => (),
                },
                _ => (),
            }
        }
    }
}
