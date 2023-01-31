use bevy::prelude::*;

// Represents the state of game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Boot,
    Menu,
    Game,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod boot;
