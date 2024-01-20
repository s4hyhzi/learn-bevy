//! A shader that uses the GLSL shading language.
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
            EguiPlugin,
        ))
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("cocos demo").show(contexts.ctx_mut(), |ui| {
        ui.heading("NB: This is a demo");
    });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(asset_server.load("branding/666.png")),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        ..default()
    });

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(
        (Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default())
    );
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
