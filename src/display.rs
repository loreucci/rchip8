use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use super::commons::CanTick;

/// Display manager
pub struct Display {
    canvas: WindowCanvas,
    memory: [u8; 64 * 32],
    refresh: bool,
    pixel_size: u32,
}

impl Display {
    /// Create a new display manager.
    pub fn new(sdl: &Sdl, pixel_size: u32) -> Result<Display, String> {
        let video_subsystem = sdl.video()?;
        let window = video_subsystem
            .window("rchip8", 64 * pixel_size, 32 * pixel_size)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Display {
            canvas,
            memory: [0; 64 * 32],
            refresh: true,
            pixel_size,
        })
    }

    /// Draw a sprite at a given location.
    #[must_use = "Value must be used to set VF"]
    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let x = x as usize;
        let y = y as usize;
        let mut carry = false;
        for v in 0..sprite.len() {
            for u in 0..8 {
                let idx = (x + u) + (y + v) * 64;
                if idx >= 64 * 32 {
                    continue;
                }
                let p = (sprite[v] >> (7 - u)) & 1;
                // check carry
                if self.memory[idx] == 1 && p ^ self.memory[idx] == 0 {
                    carry = true;
                }
                // set pixel
                self.memory[idx] ^= p;
            }
        }
        self.refresh = true;
        carry
    }

    /// Clear the display.
    pub fn clear(&mut self) {
        for i in 0..self.memory.len() {
            self.memory[i] = 0;
        }
        self.refresh = true;
    }
}

/// Each tick, the display manager refreshes the screen, if necessary.
impl CanTick for Display {
    fn tick(&mut self) {
        if !self.refresh {
            return;
        }

        // background
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // draw memory
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..64 * 32 {
            if self.memory[i] == 1 {
                let x: u32 = (i % 64).try_into().unwrap();
                let y: u32 = (i / 64).try_into().unwrap();
                self.canvas
                    .fill_rect(Rect::new(
                        (x * self.pixel_size) as i32,
                        (y * self.pixel_size) as i32,
                        self.pixel_size,
                        self.pixel_size,
                    ))
                    .unwrap_or_else(|err| {
                        eprintln!("Unable to draw: {}", err);
                    });
            }
        }

        // actual draw
        self.canvas.present();
        self.refresh = false;
    }
}
