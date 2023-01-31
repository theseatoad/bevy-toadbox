use bevy::prelude::*;

use crate::{GameState, despawn_screen};
pub struct BootPlugin;

// impl BootPlugin {
//     fn new(){

//     }
// }

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(GameState::Boot).with_system(setup))
            // While in this state, run the `countdown` system
            .add_system_set(SystemSet::on_update(GameState::Boot).with_system(countdown))
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("bevy-icon.png");
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
    // Insert the timer as a resource
    commands.insert_resource(BootTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<BootTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}
