use crate::{despawn_screen, fadeshader::FadeMaterial, GameState};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_asset_loader::prelude::*;
pub struct BootPlugin;

#[derive(AssetCollection, Resource)]
pub struct BootAssets {
    /// Name the visual source for boot screen boot-visual.png and place it in the assets/boot/ directory.
    #[asset(path = "boot/boot-visual.png")]
    pub boot_visual_asset: Handle<Image>,
    /// Name the visual source for boot screen boot-visual.png and place it in the assets/boot/ directory.
    #[asset(path = "boot/boot-background.png")]
    pub boot_background_asset: Handle<Image>,
    /// Name the audio source for boot screen boot-audio.ogg and place it in the assets/boot/ directory.
    #[asset(path = "boot/boot-audio.ogg")]
    pub boot_audio_asset: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct BootSettings {
    // Background color of the bootscreen.
    pub boot_background_color: Color,
    /// Length (in s) of the boot_screen (not including fade_in_time and fade_out_time.)
    pub length: f32,
    /// Amount of time to wait before playing audio.
    pub audio_delay: f32,
}

impl Default for BootSettings {
    fn default() -> Self {
        Self {
            boot_background_color: Color::GREEN,
            length: 3.0,
            audio_delay: 1.0,
        }
    }
}

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app.init_resource::<BootSettings>()
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(GameState::Boot).with_system(setup))
            // While in this state, run the `countdown` system
            .add_system_set(
                SystemSet::on_update(GameState::Boot)
                    .with_system(core_timer)
                    .with_system(audio_delay_timer),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameState::Boot).with_system(despawn_screen::<OnBootScreen>),
            );
    }
}
// Tag component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnBootScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct BootTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct AudioDelayTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct FadeInTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct FadeOutTimer(Timer);

fn setup(
    mut commands: Commands,
    settings: Res<BootSettings>,
    assets: Res<BootAssets>,
    mut materials: ResMut<Assets<FadeMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(settings.boot_background_color),
            },
            ..Default::default()
        })
        .insert(OnBootScreen);
    let icon = assets.boot_visual_asset.clone();
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            OnBootScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    size: Size::new(Val::Px(200.0), Val::Auto),
                    ..default()
                },
                image: UiImage(icon),
                ..default()
            });
        });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
            x: 600.0,
            y: 500.0,
            z: 1.0,
        }),
        material: materials.add(FadeMaterial {
            speed: 0.5,
            mult: 0.5,
            min: 0.0,
            max: 1.0,
            texture: assets.boot_background_asset.clone(),
        }),
        ..default()
    });
    // Insert the timer as a resource
    commands.insert_resource(BootTimer(Timer::from_seconds(
        settings.length,
        TimerMode::Once,
    )));
    commands.insert_resource(AudioDelayTimer(Timer::from_seconds(
        settings.audio_delay,
        TimerMode::Once,
    )));
}

// Tick the timer, and change state when finished
fn core_timer(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<BootTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}

fn audio_delay_timer(
    time: Res<Time>,
    mut timer: ResMut<AudioDelayTimer>,
    audio: Res<Audio>,
    assets: Res<BootAssets>,
) {
    if timer.tick(time.delta()).just_finished() {
        let music = assets.boot_audio_asset.clone();
        audio.play(music);
        timer.pause();
    }
}
