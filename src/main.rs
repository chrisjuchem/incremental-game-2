mod shader;

use crate::shader::{load_noise, MapMaterial, NoiseTex};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::primitives::Frustum;
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::window::WindowResolution;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()))
        .add_plugin(Material2dPlugin::<MapMaterial>::default())
        .add_systems(PreStartup, load_noise)
        .add_systems(Startup, add_objects)
        .add_systems(Startup, spawn_camera)
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

// === Systems ===

fn add_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MapMaterial>>,
    noise: Res<NoiseTex>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(800., 800.))))
            .into(),
        material: materials.add(MapMaterial {
            color1: Color::YELLOW,
            color2: Color::MAROON,
            noise: noise.clone().0,
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
