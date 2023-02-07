use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_toadbox::{
    mainmenu::{MainMenuAssets, MainMenuPlugin, MainMenuSettings},
    GameState,
};
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 800.0,
                        height: 600.0,
                        title: "MainMenu".to_string(),
                        ..default()
                    },
                    ..default()
                })
                .build(),
        )
        .insert_resource(MainMenuSettings {})
        .add_loading_state(
            LoadingState::new(GameState::MainMenuLoading)
                .continue_to_state(GameState::Menu)
                .with_collection::<MainMenuAssets>(),
        )
        .add_state(GameState::MainMenuLoading)
        .add_plugin(MainMenuPlugin)
        .run();
}
