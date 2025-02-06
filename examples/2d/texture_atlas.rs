//! In this example we generate four texture atlases (sprite sheets) from a folder containing
//! individual sprites.
//!
//! The texture atlases are generated with different padding and sampling to demonstrate the
//! effect of these settings, and how bleeding issues can be resolved by padding the sprites.
//!
//! Only one padded and one unpadded texture atlas are rendered to the screen.
//! An upscaled sprite from each of the four atlases are rendered to the screen.

use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct RpgSpriteFolder(Handle<LoadedFolder>);

// 1.
fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    // 获得 load_folder 的句柄,减少重复加载
    // 注意: rpg 中的任有目录嵌套
    commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("textures/rpg")));
}

// 2.
// 默认 State::Setup
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    rpg_sprite_folder: Res<RpgSpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>, // 注意这个内置的 Event类型可以用于测试是否加载完成
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    // 测试加载完成后,推进状态
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

// 3.
// 状态推进至 State::Finished
// OnEnter 仅执行一次
fn setup(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteFolder>, // 资源 Root 句柄
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>, // 资产管理器<纹理图集>
    loaded_folders: Res<Assets<LoadedFolder>>,               // 资产管理器<目录>
    mut textures: ResMut<Assets<Image>>,                     // 资产管理器<图像>
) {
    // 需要从资源管理器中获取加载的文件夹的引用(通过句柄)
    let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();

    // create texture atlases with different padding and sampling
    // 从这里开始,都是在对纹理图集进行处理,将零碎的纹理合并成一个大的纹理
    // 都是抽像的操作,并没有开始渲染
    let (texture_atlas_linear, linear_sources, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::linear()), // 平滑(模糊)
        &mut textures,
    );
    let atlas_linear_handle = texture_atlases.add(texture_atlas_linear);

    let (texture_atlas_nearest, nearest_sources, nearest_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()), // 马赛克(像素化)
        &mut textures,
    );
    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

    let (texture_atlas_linear_padded, linear_padded_sources, linear_padded_texture) =
        create_texture_atlas(
            loaded_folder,
            Some(UVec2::new(6, 6)),
            Some(ImageSampler::linear()),
            &mut textures,
        );
    let atlas_linear_padded_handle = texture_atlases.add(texture_atlas_linear_padded.clone());

    let (texture_atlas_nearest_padded, nearest_padded_sources, nearest_padded_texture) =
        create_texture_atlas(
            loaded_folder,
            Some(UVec2::new(6, 6)),
            Some(ImageSampler::nearest()),
            &mut textures,
        );
    let atlas_nearest_padded_handle = texture_atlases.add(texture_atlas_nearest_padded);
    // ----- 完成对纹理图集的处理 ----- //

    // setup 2d scene
    commands.spawn(Camera2d);

    // padded textures are to the right, unpadded to the left

    // draw unpadded texture atlas
    // -- 渲染 vendor(大图以下) 以上的纹理图集 -- //
    commands.spawn((
        Sprite::from_image(linear_texture.clone()),
        Transform {
            translation: Vec3::new(-250.0, -130.0, 0.0),
            scale: Vec3::splat(0.8),
            ..default()
        },
    ));

    // draw padded texture atlas
    commands.spawn((
        Sprite::from_image(linear_padded_texture.clone()),
        Transform {
            translation: Vec3::new(250.0, -130.0, 0.0),
            scale: Vec3::splat(0.8),
            ..default()
        },
    ));
    // ----- 完成渲染纹理图集 ----- //

    // ----- 标题处理 ----- //
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // padding label text style
    let text_style: TextFont = TextFont {
        font: font.clone(),
        font_size: 42.0,
        ..default()
    };

    // labels to indicate padding

    // No padding
    create_label(
        &mut commands,
        (-250.0, 330.0, 0.0),
        "No padding",
        text_style.clone(),
    );

    // Padding
    create_label(&mut commands, (250.0, 330.0, 0.0), "Padding", text_style);
    // ----- 完成标题处理 ----- //

    // get handle to a sprite to render
    let vendor_handle: Handle<Image> = asset_server
        .get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png")
        .unwrap();

    // configuration array to render sprites through iteration
    let configurations: [(
        &str,
        Handle<TextureAtlasLayout>,
        TextureAtlasSources,
        Handle<Image>,
        f32,
    ); 4] = [
        (
            "Linear",
            atlas_linear_handle,
            linear_sources,
            linear_texture,
            -350.0,
        ),
        (
            "Nearest",
            atlas_nearest_handle,
            nearest_sources,
            nearest_texture,
            -150.0,
        ),
        (
            "Linear",
            atlas_linear_padded_handle,
            linear_padded_sources,
            linear_padded_texture,
            150.0,
        ),
        (
            "Nearest",
            atlas_nearest_padded_handle,
            nearest_padded_sources,
            nearest_padded_texture,
            350.0,
        ),
    ];

    // label text style
    let sampling_label_style = TextFont {
        font,
        font_size: 25.0,
        ..default()
    };

    let base_y = 170.0; // y position of the sprites

    // 创建四个大 Vendor
    for (sampling, atlas_handle, atlas_sources, atlas_texture, x) in configurations {
        // render a sprite from the texture_atlas
        create_sprite_from_atlas(
            &mut commands,
            (x, base_y, 0.0),
            atlas_texture,
            atlas_sources,
            atlas_handle,
            &vendor_handle,
        );

        // render a label to indicate the sampling setting
        create_label(
            &mut commands,
            (x, base_y + 110.0, 0.0), // offset to y position of the sprite
            sampling,
            sampling_label_style.clone(),
        );
    }
}

/// Create a texture atlas with the given padding and sampling settings
/// from the individual sprites in the given folder.
/// 根据给定参数创建一个纹理图集
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());
    // 遍历文件夹中的所有句柄(handles会递归遍历子目录,并排除目录)
    for handle in folder.handles.iter() {
        // 并通用句柄转换为 Image 类型的句柄,
        // 考虑 typed_debug_unchecked 和 typed_unchecked 的区别,
        // 发布环境下 type_debug_unchecked 与 typed_unchecked 一样
        let id = handle.id().typed_unchecked::<Image>();
        // 通过句柄获取资源(这里的.get完成了有效性检查)
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        // 添加到集合中
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    // 构建纹理图集
    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();

    // 将纹理集添加到资源管理器中
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    // 更新采样设置
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    // 返回布局,源,句柄
    (texture_atlas_layout, texture_atlas_sources, texture)
}

/// Create and spawn a sprite from a texture atlas
/// 演示了从 sources 中获取纹理的信息
fn create_sprite_from_atlas(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    atlas_texture: Handle<Image>,
    atlas_sources: TextureAtlasSources,
    atlas_handle: Handle<TextureAtlasLayout>,
    vendor_handle: &Handle<Image>,
) {
    commands.spawn((
        Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            scale: Vec3::splat(3.0),
            ..default()
        },
        Sprite::from_atlas_image(
            atlas_texture,
            // 这里演示了如何从纹理图集中获取纹理,sources 的作用
            // sources 包含了纹理图集中的所有纹理的信息
            // handle 从 layout 中获取纹理的索引,并从 sources 中获取纹理的信息
            // vendor_handle 的值不是作为检索的索引,而是其附带的"文件路径"
            atlas_sources.handle(atlas_handle, vendor_handle).unwrap(),
        ),
    ));
}

/// Create and spawn a label (text)
fn create_label(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    text: &str,
    text_style: TextFont,
) {
    commands.spawn((
        Text2d::new(text),
        text_style,
        TextLayout::new_with_justify(JustifyText::Center),
        Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            ..default()
        },
    ));
}
