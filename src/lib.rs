mod animation;
mod animation_atlas;
mod bundle;
pub mod components;
mod loader;

pub use animation::Animation;
pub use animation_atlas::AnimationAtlas;
pub use bundle::AnimationBundle;
pub use components::{AnimationAtlasIndex, AnimationTimer, Features};

mod plugin {
    use super::{bundle::*, loader::AnimationLoader, Animation, AnimationAtlas};
    use bevy::prelude::*;
    pub struct Bevy2dAnimationPlugin;

    impl Plugin for Bevy2dAnimationPlugin {
        fn build(&self, app: &mut App) {
            app.add_asset::<Animation>()
                .add_asset::<AnimationAtlas>()
                .add_asset_loader(AnimationLoader)
                .init_asset_loader::<AnimationLoader>()
                .add_systems(Update, insert_animation_system)
                .add_systems(Update, update_animation_system)
                .add_systems(Update, update_frame_system);
        }
    }
}

pub use plugin::Bevy2dAnimationPlugin;
