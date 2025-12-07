//! A shader and a material that uses it.
// migration 0.16.x -> 0.17.x
// Material2d / Material2dPlugin 迁移了路径
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/custom/effect_01.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera2d);

    let mesh_handle = meshes.add(Rectangle::default());
    let material_handle = materials.add(CustomMaterial {
        color: LinearRgba::WHITE,
    });

    // material
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        Transform::default().with_scale(Vec3::splat(360.0 * 2.)),
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    // 混合模式后，shader 里 alpha 通道的值会被用来做透明度
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
