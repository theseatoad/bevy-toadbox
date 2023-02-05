use bevy::prelude::*;
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a132fdae-d598-45ac-8225-97a2a3f056e0"]
pub struct FadeMaterial {
    // Changes the speed of the fade.
    #[uniform(0)]
    pub speed: f32,
    // Sin multiplier (changes the "wave feeling" of it.)
    #[uniform(0)]
    pub mult: f32,
    #[uniform(0)]
    pub min: f32,
    #[uniform(0)]
    pub max: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for FadeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fadematerial.wgsl".into()
    }
}
