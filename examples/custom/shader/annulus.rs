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
const SHADER_ASSET_PATH: &str = "shaders/custom/annulus.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // ! 使用独立的图像自行进行采样设置
            //.set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        // .add_systems(
        //     Update,
        //     (set_texture_sampler_on_load, update_material, switch),
        // )
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d);

    // quad
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
            // color_texture: Some(texture_handle),
            // dither_pattern: Some(dither_handle),
            // ratio: Vec2::ZERO,
        })),
        Transform::default().with_scale(Vec3::splat(360.0 * 2.)),
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    // #[texture(3)]
    // #[sampler(4)]
    // dither_pattern: Option<Handle<Image>>,
    // #[uniform(5)]
    // ratio: Vec2,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    // 混合模式后，shader 里 alpha 通道的值会被用来做透明度
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
