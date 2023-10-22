use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnimationAtlasIndex(String);

impl AnimationAtlasIndex {
    pub fn new(index: &str) -> AnimationAtlasIndex {
        AnimationAtlasIndex(index.to_string())
    }

    pub fn as_str_ref(&self) -> &str {
        &self.0
    }

    pub fn set(&mut self, value: &str) {
        self.0 = value.to_string();
    }
}

#[macro_export]
macro_rules! set_index {
    ($var:ident, $val:expr) => {
        if $var.as_str_ref() != $val {
            $var.set($val);
        }
    };
}
pub use set_index;
