use super::commons::CanTick;

/// Delay timer
pub struct Timer {
    time: u8,
    ticks: u32,
    freq: u32,
}

impl Timer {
    /// Create a new delay timer.
    pub fn new(freq: u32) -> Timer {
        Timer {
            time: 0,
            ticks: 0,
            freq,
        }
    }

    // Set the timer to a given duration (at 60Hz).
    pub fn set(&mut self, duration: u8) {
        self.time = duration;
        self.ticks = duration as u32 * self.freq / 60;
    }

    // Get the value of the timer.
    pub fn get(&self) -> u8 {
        self.time
    }
}

/// Each tick, the timer is decreased by the appropriate amount, taking into account the different frequencies.
impl CanTick for Timer {
    fn tick(&mut self) {
        if self.time > 0 {
            self.ticks -= 1;
            self.time = (self.ticks * 60 / self.freq) as u8;
        }
    }
}
