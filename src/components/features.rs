use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Features {
    pub flip_x: bool,
    pub flip_y: bool,
}
