use bevy_ecs::prelude::*;
use geometry::prelude::*;

// TODO Handle familiarity by adding a Relationship component set, which can override the summary of a Visible entity. Relationships can include familiar entities like allies or known enemies, referred to by name, or short-term acquiantence with specific monsters, for example, "the bat" or "the goblin". Also allow Relationships to refer to entity classifications, rather than only one specific entity each, for instance, after seeing one bat, others can be summarized as "a bat". Some entities have variances, like "giant", and would be described as "a giant bat", or "zombie", and would be described as "a zombie bat", where prior familiarity with bats would still apply.

pub struct VisibleFeature {
    // TODO Don't embed feature descriptions directly in entities. Instead, add a feature database on the clientside, as a step during text rendering. Consider ways to solve version differences. One idea, every key has an last-update time, and the server tracks a feature-update timestamp for each client, pushing every key with an update time after that client's stamp. This simultaneously solves bandwidth issues, as well as allowing for localization.
    pub description: String,
    pub detail_level: u8,
    pub required_light_intensity: f32,
}

// TODO Handle cases where Sight entity is contained entirely inside of Visible entity. Consider adding an `internal_features` property to Visible.

#[derive(Component)]
pub struct Visible {
    pub rectangle: Rectangle,
    pub features: Vec<VisibleFeature>,
}

#[derive(Component)]
pub struct Sight {
    pub rectangle: Rectangle,
    pub detail_level: u8,
}

#[derive(Component)]
pub struct Light {
    pub rectangle: Rectangle,
    pub intensity: f32,
}
