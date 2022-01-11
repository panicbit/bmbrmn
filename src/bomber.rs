use bevy::prelude::{Color as BevyColor, *};
use rand::Rng;

use crate::{Alive, Position};

pub const SPRITE_SIZE: f32 = 30.;

#[derive(Component, Debug)]
pub struct Bomber {
    pub color: Color,
    pub max_bombs: u8,
    pub max_fire: u8,
    pub speed: u8,
}

impl Bomber {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            max_bombs: 1,
            max_fire: 1,
            speed: 1,
        }
    }
}

#[derive(Bundle)]
pub struct Bundle {
    bomber: Bomber,
    position: Position,
    alive: Alive,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Bundle {
    pub fn new(color: Color) -> Self {
        Self {
            bomber: Bomber::new(color),
            position: Position::default(),
            alive: Alive,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: color.to_bevy(),
                    custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
    Red,
    Blue,
}

impl Color {
    pub fn to_bevy(self) -> BevyColor {
        match self {
            Color::White => BevyColor::WHITE,
            Color::Black => BevyColor::BLACK,
            Color::Red => BevyColor::RED,
            Color::Blue => BevyColor::BLUE,
        }
    }
}

pub fn white() -> Bundle {
    Bundle::new(Color::White)
}

pub fn black() -> Bundle {
    Bundle::new(Color::Black)
}

pub fn red() -> Bundle {
    Bundle::new(Color::Red)
}

pub fn blue() -> Bundle {
    Bundle::new(Color::Blue)
}

pub fn random() -> Bundle {
    let mut bundle = red();

    let r = rand::thread_rng().gen_range(0. .. 1.);
    let g = rand::thread_rng().gen_range(0. .. 1.);
    let b = rand::thread_rng().gen_range(0. .. 1.);
    let color = BevyColor::rgb(r, g, b);

    bundle.sprite_bundle.sprite.color = color;

    bundle
}

pub fn render_system(
    mut query: Query<(&Position, &mut Transform), With<Bomber>>,
) {
    for (position, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        translation.x = position.x *  SPRITE_SIZE;
        translation.y = position.y * -SPRITE_SIZE;
    }
}
