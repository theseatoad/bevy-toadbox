use bevy::prelude::*;
use bevy_toadbox::{GameState, boot::BootPlugin};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor { 
        width: 800.0,
        height: 600.0,
        title: "Bootscreen".to_string(),
        ..default()
      },
      ..default()
    }))
    .add_startup_system(setup)
    .add_state(GameState::Boot)
    .add_plugin(BootPlugin)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
