use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub fn new(sdl: &Sdl) -> Result<Display, String> {
        let video_subsystem = sdl.video()?;
        let window = video_subsystem
            .window("rchip8", 640, 320)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Display { canvas })
    }

    pub fn draw(&mut self) {
        // background
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // actual draw
        self.canvas.present();
    }
}
