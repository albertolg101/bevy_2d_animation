use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use std::collections::HashMap;

use super::animation::Animation;
use super::components::AnimationAtlasIndex;

#[derive(TypeUuid, TypePath, Deref, DerefMut, Component, Debug)]
#[uuid = "7553d759-1915-4fc3-a8d2-51d623c8a10d"]
pub struct AnimationAtlas(HashMap<String, Handle<Animation>>);

impl AnimationAtlas {
    pub fn new() -> AnimationAtlas {
        AnimationAtlas(HashMap::new())
    }

    pub fn get_animation<'a>(
        &'a self,
        index: &'a AnimationAtlasIndex,
        animations: &'a Res<Assets<Animation>>,
    ) -> Option<&'a Animation> {
        let anim_handle = self.get(index.as_str_ref()).expect(&format!(
            "animation id {:?} does not exist in {:?}",
            index, self.0
        ));

        animations.get(&anim_handle)
    }
}
