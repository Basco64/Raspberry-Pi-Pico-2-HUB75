use crate::hub75::{Hub75, Outputs};

pub trait Animation<PINS: Outputs> {
    fn tick(&mut self, display: &mut Hub75<PINS>);
}

pub enum ActiveAnimation {
    None,
    Idle(crate::idle::IdleFallback),
    Text(crate::text::ScrollingText),
}

impl ActiveAnimation {
    pub fn step_us(&self) -> Option<u64> {
        match self {
            ActiveAnimation::None => None,
            ActiveAnimation::Idle(a) => Some(a.step_us()),
            ActiveAnimation::Text(a) => Some(a.step_us()),
        }
    }

    pub fn tick<PINS: Outputs>(&mut self, display: &mut Hub75<PINS>) {
        match self {
            ActiveAnimation::None => {}
            ActiveAnimation::Idle(a) => a.tick(display),
            ActiveAnimation::Text(a) => a.tick(display),
        }
    }
}
