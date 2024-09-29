use macroquad::prelude::*;

pub struct Ship {
    pub x_pos: u16,
}

pub struct Alien<'a> {
    pub pos: U16Vec2,
    pub tex: &'a Texture2D,
    pub zapped: bool,
}
