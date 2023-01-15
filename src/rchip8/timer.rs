use super::commons::CanTick;

pub struct Timer {
    pub time: u8,
}

impl CanTick for Timer {
    fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        }
    }
}
