use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, TextureFormat};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "6a92ef56-13b1-437a-91f5-bcf53be7b1c6"]
pub struct MapMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub noise: Handle<Image>,
}

impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/map_material.wgsl".into())
    }
}

#[derive(Resource, Deref)]
pub struct ColorTex(pub Handle<Image>);
#[derive(Resource, Deref)]
pub struct NoiseTex(pub Handle<Image>);

pub fn load_textures(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(ColorTex(assets.load("textures/colors2.png")));

    let noise_tex = assets.load("textures/3channoise.png");
    commands.insert_resource(NoiseTex(noise_tex));
}

pub fn fix_textures(mut textures: ResMut<Assets<Image>>, noise: Res<NoiseTex>) {
    let noise_img = textures.get_mut(&noise).unwrap();
    noise_img.texture_descriptor.format = TextureFormat::Rgba8Unorm;
}

pub fn textures_just_loaded(
    mut has_run: Local<bool>,
    noise: Res<NoiseTex>,
    textures: Res<Assets<Image>>,
) -> bool {
    if *has_run {
        return false;
    }
    let should_run = textures.contains(&noise);
    *has_run = should_run;
    return should_run;
}
