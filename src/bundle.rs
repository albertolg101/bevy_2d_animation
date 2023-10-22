use bevy::prelude::*;

use super::components::{AnimationAtlasIndex, AnimationTimer, Features};
use super::{Animation, AnimationAtlas};

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation_atlas: Handle<AnimationAtlas>,
    pub index: AnimationAtlasIndex,
    pub timer: AnimationTimer,
    pub features: Features,
}

pub fn insert_animation_system(
    query: Query<
        (Entity, &Handle<AnimationAtlas>, &AnimationAtlasIndex),
        (Without<Handle<TextureAtlas>>, Without<TextureAtlasSprite>),
    >,
    animations: Res<Assets<Animation>>,
    animation_atlases: Res<Assets<AnimationAtlas>>,
    mut commands: Commands,
) {
    for (entity, anim_atlas, anim_atlas_index) in &query {
        if let Some(anim_atlas) = animation_atlases.get(anim_atlas) {
            if let Some(anim) = anim_atlas.get_animation(anim_atlas_index, &animations) {
                commands.entity(entity).insert(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: anim.texture_atlas.clone(),
                    ..default()
                });
            }
        }
    }
}

pub fn update_animation_system(
    mut query: Query<
        (
            &Handle<AnimationAtlas>,
            &AnimationAtlasIndex,
            &mut AnimationTimer,
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
        ),
        Or<(
            (Added<Handle<TextureAtlas>>, Added<TextureAtlasSprite>),
            Changed<AnimationAtlasIndex>,
        )>,
    >,
    animations: Res<Assets<Animation>>,
    animation_atlases: Res<Assets<AnimationAtlas>>,
) {
    for (anim_atlas, anim_atlas_index, mut anim_timer, mut tex_atlas, mut tex_atlas_sprite) in
        &mut query
    {
        if let Some(anim_atlas) = animation_atlases.get(anim_atlas) {
            if let Some(anim) = anim_atlas.get_animation(anim_atlas_index, &animations) {
                *tex_atlas = anim.texture_atlas.clone();
                tex_atlas_sprite.index = 0;
                *anim_timer = AnimationTimer::from_seconds(anim.frame_time, TimerMode::Repeating);
            }
        }
    }
}

pub fn update_frame_system(
    mut query: Query<(
        &Handle<AnimationAtlas>,
        &AnimationAtlasIndex,
        &Features,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    animations: Res<Assets<Animation>>,
    animation_atlases: Res<Assets<AnimationAtlas>>,
    time: Res<Time>,
) {
    for (anim_atlas, anim_atlas_index, features, mut anim_timer, mut tex_atlas_sprite) in &mut query
    {
        if let Some(anim_atlas) = animation_atlases.get(anim_atlas) {
            if let Some(anim) = anim_atlas.get_animation(anim_atlas_index, &animations) {
                anim_timer.tick(time.delta());
                if anim_timer.just_finished() {
                    tex_atlas_sprite.index = if anim.is_loop {
                        (tex_atlas_sprite.index + 1) % anim.len
                    } else {
                        std::cmp::min(anim.len - 1, tex_atlas_sprite.index + 1)
                    };
                    anim_timer.reset();
                }

                tex_atlas_sprite.flip_x = anim.can_flip_x && features.flip_x;
                tex_atlas_sprite.flip_y = anim.can_flip_y && features.flip_y;
            }
        }
    }
}
