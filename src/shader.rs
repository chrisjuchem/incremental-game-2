use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid, Uuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "6a92ef56-13b1-437a-91f5-bcf53be7b1c6"]
pub struct MapMaterial {
    #[uniform(0)]
    pub color1: Color,
    #[uniform(1)]
    pub color2: Color,
    #[texture(2)]
    #[sampler(3)]
    pub noise: Handle<Image>,
}

impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/map_material.wgsl".into())
    }
}

#[derive(Resource, Clone)]
pub struct NoiseTex(pub Handle<Image>);

pub fn load_noise(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(NoiseTex(assets.load("textures/noise.png")));
}
