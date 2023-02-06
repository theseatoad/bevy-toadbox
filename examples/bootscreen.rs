use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_toadbox::{
    boot::{BootAssets, BootPlugin, BootSettings},
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
            boot_background_color: Color::rgb_linear(115.0 / 255.0, 23.0 / 255.0, 45.0 / 255.0),
            length: 4.0,
            audio_delay: 0.2,
            bg_scale: Vec3::ONE,
        })
        .add_loading_state(
            LoadingState::new(GameState::BootLoading)
                .continue_to_state(GameState::Boot)
                .with_collection::<BootAssets>(),
        )
        .add_state(GameState::BootLoading)
        .add_plugin(BootPlugin)
        .run();
}
