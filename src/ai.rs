use bevy::prelude::*;
use rand::prelude::*;
use rand::distributions;

#[derive(Component)]
pub struct Controller {
    pub timer: Timer,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, true),
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let index: u8 = rng.gen_range(0..4);

        match index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}