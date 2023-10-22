use bevy::prelude::*;
use bevy_2d_animation::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Bevy2dAnimationPlugin))
        .add_systems(Startup, spawn)
        .add_systems(PreUpdate, input)
        .run();
}

#[derive(Component)]
struct Wizard;

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let animation_atlas: Handle<AnimationAtlas> =
        asset_server.load("sprites/wizard/wizard.anim.ron");
    commands.spawn((
        Wizard,
        AnimationBundle {
            animation_atlas,
            index: AnimationAtlasIndex::new("Idle"),
            timer: AnimationTimer::default(),
            features: Features::default(),
        },
        Name::new("wizard"),
    ));
}

fn input(
    mut query: Query<(&mut AnimationAtlasIndex, &mut Features), With<Wizard>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut index, mut features) = query
        .get_single_mut()
        .expect("You can control only one wizard at a time");

    if !input.any_pressed(vec![KeyCode::A, KeyCode::D, KeyCode::S]) {
        set_index!(index, "Idle");
    }

    for key in input.get_pressed() {
        match key {
            KeyCode::D => {
                set_index!(index, "Run");
                features.flip_x = false;
            }
            KeyCode::A => {
                set_index!(index, "Run");
                features.flip_x = true;
            }
            KeyCode::S => set_index!(index, "Die"),
            _ => (),
        }
    }
}
