use bevy::ecs::prelude::*;
use bevy::math::Rect;

use crate::components::vision::{Light, Visible, Vision};

// WIP
fn vision_system(
    mut vision_query: Query<&mut Vision>,
    mut visible_query: Query<&Visible>,
    light_query: Query<&Light>,
) {
    for (visible, mut vision) in query.iter_mut() {
        // Check if there is any intervening light
        let mut is_visible = true;
        for light in light_query.iter() {
            // Calculate the distance between the visible and light sources
            let distance = visible.rect.distance_to(light.rect.center());

            // Check if the light intensity is enough to illuminate the visible entity
            if distance < light.intensity {
                is_visible = false;
                break;
            }
        }
    }
}
