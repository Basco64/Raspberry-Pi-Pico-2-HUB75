use crate::hub75::{Hub75, Outputs};

pub trait Animation<PINS: Outputs> {
    fn tick(&mut self, display: &mut Hub75<PINS>);
}

/// Pour ajouter une animation : créez son module avec une struct qui a une
/// méthode `step_us(&self) -> u64` et implémente `Animation<PINS>`,
/// ajoutez une variante ici + une ligne dans les deux match ci-dessous,
/// et une ligne dans main.rs pour la démarrer sur la bonne commande.
pub enum ActiveAnimation {
    None,
    Random(crate::tests::RandomLoop),
    Text(crate::text::ScrollingText),
    Phrases(crate::text::PhraseRotation),
    Pacman(crate::named::pacman::Pacman),
    Squid(crate::space_invaders::squid::Squid),
    Crab(crate::space_invaders::crab::Crab),
}

impl ActiveAnimation {
    pub fn step_us(&self) -> Option<u64> {
        match self {
            ActiveAnimation::None => None,
            ActiveAnimation::Random(a) => Some(a.step_us()),
            ActiveAnimation::Text(a) => Some(a.step_us()),
            ActiveAnimation::Phrases(a) => Some(a.step_us()),
            ActiveAnimation::Pacman(a) => Some(a.step_us()),
            ActiveAnimation::Squid(a) => Some(a.step_us()),
            ActiveAnimation::Crab(a) => Some(a.step_us()),
        }
    }

    pub fn tick<PINS: Outputs>(&mut self, display: &mut Hub75<PINS>) {
        match self {
            ActiveAnimation::None => {}
            ActiveAnimation::Random(a) => a.tick(display),
            ActiveAnimation::Text(a) => a.tick(display),
            ActiveAnimation::Phrases(a) => a.tick(display),
            ActiveAnimation::Pacman(a) => a.tick(display),
            ActiveAnimation::Squid(a) => a.tick(display),
            ActiveAnimation::Crab(a) => a.tick(display),
        }
    }
}
