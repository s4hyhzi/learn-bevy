//! A shader that uses the GLSL shading language.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
            MaterialPlugin::<LightMaterial>::default(),
        ))
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: (
        ResMut<Assets<CustomMaterial>>,
        ResMut<Assets<LightMaterial>>,
    ),
) {
    let mut custom_material = materials.0;
    let mut light_material = materials.1;

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: custom_material.add(CustomMaterial {
            color: Color::Rgba {
                red: 1.0,
                green: 0.5,
                blue: 0.3,
                alpha: 1.00,
            },
            light_color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.00,
            },
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });

    // light
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        transform: Transform::from_xyz(1.2, 1.0, 2.0),
        material: light_material.add(LightMaterial {}),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    light_color: Color,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct LightMaterial {}

impl Material for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lighting_material.wgsl".into()
    }
}
