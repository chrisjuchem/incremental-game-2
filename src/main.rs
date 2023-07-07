mod factory;
mod recipe;
mod shader;

use crate::factory::{load_factory_sprite, spawn_factories, tick_factories};
use crate::recipe::{Materials, Recipes};
use crate::shader::{
    fix_textures, load_textures, textures_just_loaded, ColorTex, MapMaterial, NoiseTex,
};
use bevy::asset::ChangeWatcher;
use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy::window::WindowResolution;
use std::time::Duration;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()).set(asset_plugin()))
        .add_plugins(Material2dPlugin::<MapMaterial>::default())
        .add_systems(PreStartup, load_textures)
        .add_systems(Update, fix_textures.run_if(textures_just_loaded))
        .add_systems(Startup, add_objects)
        .add_systems(Startup, spawn_camera)
        .insert_resource(Recipes::default())
        .insert_resource(Materials::default_populated())
        .add_systems(PreStartup, load_factory_sprite)
        .add_systems(Startup, spawn_factories)
        .add_systems(FixedUpdate, tick_factories)
        .run()
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
            ..Window::default()
        }),
        close_when_requested: true,
        ..Default::default()
    }
}

fn asset_plugin() -> AssetPlugin {
    AssetPlugin {
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(0)),
        ..Default::default()
    }
}

// === Systems ===

fn add_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MapMaterial>>,
    color: Res<ColorTex>,
    noise: Res<NoiseTex>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(800., 800.))))
            .into(),
        material: materials.add(MapMaterial {
            color: color.clone(),
            noise: noise.clone(),
        }),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            area: Rect {
                min: Vec2 {
                    x: -RESOLUTION,
                    y: -1.,
                },
                max: Vec2 {
                    x: RESOLUTION,
                    y: 1.,
                },
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
