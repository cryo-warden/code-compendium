use bevy_ecs::prelude::*;
use geometry::prelude::*;

pub struct SmellFeature {
    pub description: String,
    pub intensity: f32, // Only the highest intensity features are detected.
}

#[derive(Component)]
pub struct Smellable {
    pub rectangle: Rectangle,
    pub features: Vec<SmellFeature>,
}

#[derive(Component)]
pub struct Smell {
    pub rectangle: Rectangle,
    pub complexity: u8, // The number of features that can be detected.
    pub minimum_intensity: f32,
}
