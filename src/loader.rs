use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use super::{Animation, AnimationAtlas};

#[derive(Default)]
pub struct AnimationLoader;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationIniter {
    pub id: String,
    pub fps: f32,
    pub tile_size: Vec2,
    pub rows: usize,
    pub columns: usize,
    pub texture_path: String,
    pub is_loop: bool,
    #[serde(default)]
    pub can_flip_x: bool,
    #[serde(default)]
    pub can_flip_y: bool,
}

impl AssetLoader for AnimationLoader {
    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { load_animation(bytes, load_context).await })
    }
}

async fn load_animation<'de, 'a, 'b>(
    bytes: &'a [u8],
    context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    let animations_initers: Vec<AnimationIniter> = ron::de::from_bytes(bytes)?;
    let mut animation_atlas = AnimationAtlas::new();

    for animation_initer in animations_initers {
        // TODO: handle unwrap more safety
        let image_path = context.path().parent().unwrap().join(
            // TODO: handle unwrap more safety
            //"texture_path {} on object {}##{} points to a path that does not exists",
            PathBuf::from_str(&animation_initer.texture_path).unwrap(),
        );

        // TODO: handle unwrap more safety
        let image_handle: Handle<Image> = context.get_handle(image_path.to_str().unwrap());

        let texture_atlas = TextureAtlas::from_grid(
            image_handle,
            animation_initer.tile_size,
            animation_initer.columns,
            animation_initer.rows,
            None,
            None,
        );

        let atlas_handler = context.set_labeled_asset(
            &format!("TextureAtlas/{}", animation_initer.id),
            LoadedAsset::new(texture_atlas).with_dependency(image_path.into()),
        );

        let animation_settings = Animation {
            len: animation_initer.rows * animation_initer.columns,
            frame_time: 1. / animation_initer.fps,
            texture_atlas: atlas_handler,
            is_loop: animation_initer.is_loop,
            can_flip_x: animation_initer.can_flip_x,
            can_flip_y: animation_initer.can_flip_y,
        };

        let animation_handle = context.set_labeled_asset(
            &format!("Animation/{}", animation_initer.id),
            LoadedAsset::new(animation_settings),
        );

        animation_atlas.insert(animation_initer.id, animation_handle);
    }
    context.set_default_asset(LoadedAsset::new(animation_atlas));

    Ok(())
}
