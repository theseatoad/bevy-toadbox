use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_asset_loader::prelude::*;
use bevy_toadbox::{
    boot::{BootAssets, BootPlugin, BootSettings},
    fadeshader::FadeMaterial,
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
                        title: "Bootscreen".to_string(),
                        ..default()
                    },
                    ..default()
                })
                .build(),
        )
        .insert_resource(BootSettings {
            boot_background_color: Color::WHITE,
            length: 3.0,
            audio_delay: 1.0,
        })
        .add_loading_state(
            LoadingState::new(GameState::BootLoading)
                .continue_to_state(GameState::Boot)
                .with_collection::<BootAssets>(),
        )
        .add_plugin(Material2dPlugin::<FadeMaterial>::default())
        .add_state(GameState::BootLoading)
        .add_plugin(BootPlugin)
        .run();
}
