use bevy_ecs::prelude::*;
use geometry::prelude::*;

pub struct AudibleFeature {
    pub description: String,
    pub detail_level: u8,
    pub pitch: f32,
    pub intensity: f32,
}

#[derive(Component)]
pub struct Audible {
    pub rectangle: Rectangle,
    pub features: Vec<AudibleFeature>,
}

#[derive(Component)]
pub struct Hearing {
    pub rectangle: Rectangle,
    pub detail_level: u8,
    pub minimum_intensity: f32,
}

#[derive(Component)]
pub struct Acoustics {
    pub rectangle: Rectangle,
    pub muffling_factor: f32,
}
