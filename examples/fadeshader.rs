//! Based on the shader_material.rs example at https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_material.rs
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use bevy::{
    asset::{AssetServer, Assets},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    utils::default,
};
use bevy_toadbox::fadeshader::FadeMaterial;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(Material2dPlugin::<FadeMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FadeMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(FadeMaterial {
            speed: 1.0,
            mult: 3.0,
            min: 0.2,
            max: 0.8,
            texture: asset_server.load("boot/boot-visual.png"),
        }),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
}
