use super::commons::CanTick;

pub struct Timer {
    time: u8,
    ticks: u32,
    freq: u32,
}

impl Timer {
    pub fn new(freq: u32) -> Timer {
        Timer {
            time: 0,
            ticks: 0,
            freq,
        }
    }

    pub fn set(&mut self, duration: u8) {
        self.time = duration;
        self.ticks = duration as u32 * self.freq / 60;
    }

    pub fn get(&self) -> u8 {
        self.time
    }
}

impl CanTick for Timer {
    fn tick(&mut self) {
        if self.time > 0 {
            self.ticks -= 1;
            self.time = (self.ticks * 60 / self.freq) as u8;
        }
    }
}
