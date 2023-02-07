use bevy::prelude::*;
// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    BootLoading,
    Boot,
    MainMenuLoading,
    Menu,
    GameLoading,
    Game,
}

impl States for GameState {
    type Iter = std::array::IntoIter<GameState, 6>;

    fn variants() -> Self::Iter {
        [
            GameState::BootLoading,
            GameState::Boot,
            GameState::MainMenuLoading,
            GameState::Menu,
            GameState::GameLoading,
            GameState::Game,
        ]
        .into_iter()
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod boot;
pub mod fadeshader;
pub mod mainmenu;
