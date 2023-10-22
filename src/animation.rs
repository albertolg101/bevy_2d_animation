use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

#[derive(TypeUuid, TypePath, Debug)]
#[uuid = "5750daca-7364-4412-bb14-4a7de71e0350"]
pub struct Animation {
    pub texture_atlas: Handle<TextureAtlas>,
    pub len: usize,
    pub frame_time: f32,
    pub is_loop: bool,
    pub can_flip_x: bool,
    pub can_flip_y: bool,
}
