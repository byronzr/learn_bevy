//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::{
            binding_types::{texture_storage_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::GpuImage,
    },
    shader::PipelineCacheError,
};
use std::borrow::Cow;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/game_of_life.wgsl";

// const DISPLAY_FACTOR_FOR_WINDOW: u32 = 4;
// const DISPLAY_FACTOR_FOR_SPRITE: u32 = 4;
// // sprite image size
// const SIZE: (u32, u32) = (
//     1280 / DISPLAY_FACTOR_FOR_WINDOW,
//     720 / DISPLAY_FACTOR_FOR_WINDOW,
// );
// const WORKGROUP_SIZE: u32 = 8;

// since 0.17.0
const DISPLAY_FACTOR: u32 = 4;
const SIZE: UVec2 = UVec2::new(1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // resolution: (
                        //     (SIZE.0 * DISPLAY_FACTOR_FOR_WINDOW) as f32,
                        //     (SIZE.1 * DISPLAY_FACTOR_FOR_WINDOW) as f32,
                        // )
                        //     .into(),
                        // since 0.17.0
                        resolution: (SIZE * DISPLAY_FACTOR).into(),
                        // uncomment for unthrottled FPS
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GameOfLifeComputePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, switch_textures)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // ! different v0.16.1
    // ! R32Float => Rgba32Float
    // let mut image = Image::new_target_texture(SIZE.0, SIZE.1, TextureFormat::Rgba32Float);
    // since 0.17.0
    let mut image = Image::new_target_texture(SIZE.x, SIZE.y, TextureFormat::Rgba32Float);
    // ! render world need read and write access to the texture
    image.asset_usage = RenderAssetUsages::RENDER_WORLD;

    // TextureUsages::COPY_DST  // 允许作为目标进行拷贝 (被写入)
    // TextureUsages::COPY_SRC // 允许作为源进行拷贝 (被读取)
    // TextureUsages::STORAGE_BINDING  允许作为存储纹理绑定 (textureSample/textureLoad 读取)
    // TextureUsages::TEXTURE_BINDING; 允许作为纹理绑定 被计算/片元/顶点阶段读写（受平台限制）
    // TextureUsages::RENDER_ATTACHMENT (颜色/深度/模板,Render Pipeline Write)
    // TextureUsages::STORAGE_ATOMIC (原子操作)
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image0 = images.add(image.clone());
    let image1 = images.add(image);

    commands.spawn((
        Sprite {
            image: image0.clone(),
            //custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            custom_size: Some(SIZE.as_vec2()), // since 0.17.0
            ..default()
        },
        // Transform::from_scale(Vec3::splat(DISPLAY_FACTOR_FOR_SPRITE as f32)),
        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)), // since 0.17.0
    ));
    commands.spawn(Camera2d);

    commands.insert_resource(GameOfLifeImages {
        texture_a: image0,
        texture_b: image1,
    });

    commands.insert_resource(GameOfLifeUniforms {
        alive_color: LinearRgba::RED,
    });
}

// Switch texture to display every frame to show the one that was written to most recently.
fn switch_textures(images: Res<GameOfLifeImages>, mut sprite: Single<&mut Sprite>) {
    if sprite.image == images.texture_a {
        // ! different v0.16.1
        // ! clone_weak() => clone()
        sprite.image = images.texture_b.clone();
    } else {
        sprite.image = images.texture_a.clone();
    }
}

struct GameOfLifeComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct GameOfLifeLabel;

/// ! The extension for the Game of Life compute shader.
/// ! Plugin -> render world -> render graph -> node(GameOfLifeNode)
/// ! config the node in the render graph(edge)
impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins((
            ExtractResourcePlugin::<GameOfLifeImages>::default(),
            ExtractResourcePlugin::<GameOfLifeUniforms>::default(),
        ));

        // ! !app is the main world
        // ! render_app is render world
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(RenderStartup, init_game_of_life_pipeline)
            .add_systems(
                Render,
                // ! different v0.16.1
                // ! RenderSet => RenderSystem
                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
            );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(GameOfLifeLabel, GameOfLifeNode::default());
        render_graph.add_node_edge(GameOfLifeLabel, bevy::render::graph::CameraDriverLabel);
    }
    // ! different v0.16.1
    // ! cancel finish()
    // ! pipeline init in build (RenderStartup)
}

#[derive(Resource, Clone, ExtractResource)]
struct GameOfLifeImages {
    texture_a: Handle<Image>,
    texture_b: Handle<Image>,
}

#[derive(Resource, Clone, ExtractResource, ShaderType)]
struct GameOfLifeUniforms {
    alive_color: LinearRgba,
}

#[derive(Resource)]
struct GameOfLifeImageBindGroups([BindGroup; 2]);

/// ! preparse bind group for the compute shader
fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<GameOfLifePipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    game_of_life_images: Res<GameOfLifeImages>,
    game_of_life_uniforms: Res<GameOfLifeUniforms>,
    render_device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    // ! ping-pong execution has completed on the switch_textures (ecs::Update)
    // ! get texture view from resource handle
    // ! get GpuImage from RenderAssets
    let view_a = gpu_images.get(&game_of_life_images.texture_a).unwrap();
    let view_b = gpu_images.get(&game_of_life_images.texture_b).unwrap();

    // Uniform buffer is used here to demonstrate how to set up a uniform in a compute shader
    // Alternatives such as storage buffers or push constants may be more suitable for your use case
    let mut uniform_buffer = UniformBuffer::from(game_of_life_uniforms.into_inner());
    uniform_buffer.write_buffer(&render_device, &queue);

    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            // ! raw TextureView wrap in GpuImage
            &view_a.texture_view,
            &view_b.texture_view,
            &uniform_buffer,
        )),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &view_b.texture_view,
            &view_a.texture_view,
            &uniform_buffer,
        )),
    );
    commands.insert_resource(GameOfLifeImageBindGroups([bind_group_0, bind_group_1]));
}

#[derive(Resource)]
struct GameOfLifePipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

/// ! different v0.16.1
/// ! replace <trait FromWorld>
/// ! --
/// ! insert resource(GameOfLifePipeline)
/// ! init the compute shader pipeline
/// ! pipeline(init)
/// ! pipeline(update)
fn init_game_of_life_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let texture_bind_group_layout = render_device.create_bind_group_layout(
        "GameOfLifeImages",
        // ! sequential is used to define the order of bindings
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                // binding(0)
                texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadOnly),
                // binding(1)
                texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::WriteOnly),
                // color // binding(2)
                uniform_buffer::<GameOfLifeUniforms>(false),
            ),
        ),
    );
    let shader = asset_server.load(SHADER_ASSET_PATH);
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        layout: vec![texture_bind_group_layout.clone()],
        shader: shader.clone(),
        entry_point: Some(Cow::from("init")),
        ..default()
    });
    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        layout: vec![texture_bind_group_layout.clone()],
        shader,
        entry_point: Some(Cow::from("update")),
        ..default()
    });

    commands.insert_resource(GameOfLifePipeline {
        texture_bind_group_layout,
        init_pipeline,
        update_pipeline,
    });
}

enum GameOfLifeState {
    Loading,
    Init,
    Update(usize),
}

struct GameOfLifeNode {
    state: GameOfLifeState,
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}

impl render_graph::Node for GameOfLifeNode {
    // ! CPU stage
    // ! after Update(ecs)
    // ! after RenderSystem::PrepareBindGroups(Render)
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GameOfLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            GameOfLifeState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = GameOfLifeState::Init;
                    }
                    // If the shader hasn't loaded yet, just wait.
                    CachedPipelineState::Err(PipelineCacheError::ShaderNotLoaded(_)) => {}
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = GameOfLifeState::Update(1);
                }
            }
            // ! swap the bind groups for the next update
            // ! ping
            GameOfLifeState::Update(0) => {
                self.state = GameOfLifeState::Update(1);
            }
            // ! pong
            GameOfLifeState::Update(1) => {
                self.state = GameOfLifeState::Update(0);
            }
            GameOfLifeState::Update(_) => unreachable!(),
        }
    }

    // ! GPU stage
    // ! like begin_compute_pass
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<GameOfLifeImageBindGroups>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<GameOfLifePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                // pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
                pass.dispatch_workgroups(SIZE.x / WORKGROUP_SIZE, SIZE.y / WORKGROUP_SIZE, 1); // since 0.17.0
            }
            GameOfLifeState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[]);
                pass.set_pipeline(update_pipeline);
                // pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
                pass.dispatch_workgroups(SIZE.x / WORKGROUP_SIZE, SIZE.y / WORKGROUP_SIZE, 1); // since 0.17.0
            }
        }

        Ok(())
    }
}
