use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Deref, DerefMut, Default)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(duration: Duration, mode: TimerMode) -> AnimationTimer {
        AnimationTimer(Timer::new(duration, mode))
    }

    pub fn from_seconds(duration: f32, mode: TimerMode) -> AnimationTimer {
        AnimationTimer(Timer::from_seconds(duration, mode))
    }
}
