//! A shader and a material that uses it.
// migration 0.16.x -> 0.17.x
// Material2d / Material2dPlugin 迁移了路径
use bevy::{
    image::ImageSampler,
    input::mouse::MouseWheel,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/custom/dither.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // ! 使用独立的图像自行进行采样设置
            //.set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (set_texture_sampler_on_load, update_material, switch),
        )
        .run();
}

// switch dither pattern or change dither ratio
fn switch(
    mut images_resource: ResMut<ImageResource>,
    asset_server: Res<AssetServer>,
    mut idx: Local<usize>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    // 新增参数：读取鼠标滚轮事件
    mut scroll_evr: MessageReader<MouseWheel>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        *idx += 1;
        if *idx > 2 {
            *idx = 0;
        }
        let path = format!("dither/HDR_L_{}.png", *idx);
        let dither_handle = asset_server.load(path);
        images_resource.dither_pattern = dither_handle.clone();
        images_resource.dither_size = Vec2::ZERO;
        for (_, material) in materials.iter_mut() {
            material.dither_pattern = Some(dither_handle.clone());
        }
    }

    // --- 新增：处理鼠标滚轮的部分 ---
    for event in scroll_evr.read() {
        images_resource.threshold = images_resource.threshold.clamp(1., 100.0);
        // UP = -y
        // DOWN = +y
        // so we need to invert the y value
        let y = -f32::copysign(1.0, event.y);
        let scroll_amount = event.y.abs().ceil() * y;
        images_resource.threshold += scroll_amount;
        images_resource.threshold = images_resource.threshold.clamp(1., 100.0);
    }
}

#[derive(Debug, Resource, Default)]
struct ImageResource {
    pub image: Handle<Image>,
    pub image_size: Vec2,
    pub dither_pattern: Handle<Image>,
    pub dither_size: Vec2,
    pub threshold: f32,
}

// Assset load image not synchronous, so we need to set the sampler after the image is loaded
fn set_texture_sampler_on_load(
    mut asset_events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    mut images_resource: ResMut<ImageResource>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            // image
            if images_resource.image.id() == *id {
                if let Some(image) = images.get_mut(*id) {
                    image.sampler = ImageSampler::nearest();
                    images_resource.image_size = image.size_f32();
                    //dither_image.texture_descriptor.format = TextureFormat::R8Unorm;
                }
            }

            // dither pattern
            if images_resource.dither_pattern.id() == *id {
                if let Some(image) = images.get_mut(*id) {
                    image.sampler = ImageSampler::nearest();
                    images_resource.dither_size = image.size_f32();
                    // ! 优化狂人总喜欢用单通道的纹理(Gray)
                    // image.texture_descriptor.format = TextureFormat::R8Unorm;
                }
            }
        }
    }
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d);

    let texture_handle = asset_server.load("dither/cat01.png");
    let dither_handle = asset_server.load("dither/HDR_L_0.png");

    commands.insert_resource(ImageResource {
        image: texture_handle.clone(),
        dither_pattern: dither_handle.clone(),
        threshold: 1.0,
        ..default()
    });

    // quad
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
            color_texture: Some(texture_handle),
            dither_pattern: Some(dither_handle),
            ratio: Vec2::ZERO,
        })),
        Transform::default().with_scale(Vec3::splat(360.0 * 2.)),
    ));
}

// update the material with the dither pattern and ratio
fn update_material(
    mut materials: ResMut<Assets<CustomMaterial>>,
    images_resource: Res<ImageResource>,
) {
    for (_, material) in materials.iter_mut() {
        material.dither_pattern = Some(images_resource.dither_pattern.clone());
        material.ratio = if images_resource.dither_size != Vec2::ZERO
            && images_resource.image_size != Vec2::ZERO
        {
            images_resource.image_size / images_resource.dither_size * images_resource.threshold
        } else {
            Vec2::ZERO
        };
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    dither_pattern: Option<Handle<Image>>,
    #[uniform(5)]
    ratio: Vec2,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    // fn alpha_mode(&self) -> AlphaMode2d {
    //     AlphaMode2d::Mask(0.5)
    // }
}
