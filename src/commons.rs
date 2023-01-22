/// Common trait for components that have to do something each cycle.
pub trait CanTick {
    /// Perform actions for this cycle.
    fn tick(&mut self);
}
