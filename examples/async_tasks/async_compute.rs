//! This example shows how to use the ECS and the [`AsyncComputeTaskPool`]
//! to spawn, poll, and complete tasks across systems and system ticks.
//!
//! 这个个例子中重点展示了如何使用 ECS 与 [`AsyncComputeTaskPool`] 来生成,轮询,完成任务
//! 1.  用 Component() 包装一个 Task<CommandQueue> 任务,置入世界
//! 2.  AsyncComputeTaskPool.spawn 生成一个异步任务,但是该闭包已经逃逸到 System 之外,已经无法获得 Command 与 World
//!     所以让其返回的时一个 CommandQueue,在 CommandQueue 中,再次以闭包的方式,将实际的 Entity 操作放入队列
//! 3. CommmandQueue,是一个高密度高效的异步命令队列,在这个队列中,可以获取到 World 的可写引用,以此来操作 Entity
//! 4. 至止,已经完成了 Task<CommandQueue> 的异步任务的构建.
//! 5. handle_tasks() 系统,轮询所有未完成的 Task<CommandQueue>
//!     block_on,以阻塞方式执行异步函数,而 future::poll_once 以 Option 的方式快速响应(阻塞)
//!     在成功获得 CommandQueue 后,将其以 commands.append 的方式添加到 World 中,以此来执行实际的 Entity 操作

use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};

use rand::Rng;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_env, add_assets, spawn_tasks))
        .add_systems(Update, handle_tasks)
        .run();
}

// Number of cubes to spawn across the x, y, and z axis
const NUM_CUBES: u32 = 6;

#[derive(Resource, Deref)]
struct BoxMeshHandle(Handle<Mesh>);

#[derive(Resource, Deref)]
struct BoxMaterialHandle(Handle<StandardMaterial>);

/// Startup system which runs only once and generates our Box Mesh
/// and Box Material assets, adds them to their respective Asset
/// Resources, and stores their handles as resources so we can access
/// them later when we're ready to render our Boxes
fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh_handle = meshes.add(Cuboid::new(0.25, 0.25, 0.25));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));

    let box_material_handle = materials.add(Color::srgb(1.0, 0.2, 0.3));
    commands.insert_resource(BoxMaterialHandle(box_material_handle));
}

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

/// This system generates tasks simulating computationally intensive
/// work that potentially spans multiple frames/ticks. A separate
/// system, [`handle_tasks`], will poll the spawned tasks on subsequent
/// frames/ticks, and use the results to spawn cubes
fn spawn_tasks(mut commands: Commands) {
    // ! 获得一个线程池的(引用)
    let thread_pool = AsyncComputeTaskPool::get();
    for x in 0..NUM_CUBES {
        for y in 0..NUM_CUBES {
            for z in 0..NUM_CUBES {
                // Spawn new task on the AsyncComputeTaskPool; the task will be
                // executed in the background, and the Task future returned by
                // spawn() can be used to poll for the result
                // ! 提前生成一个 entity, 这个 entity 将以低耗的方式置入到 World 中
                let entity = commands.spawn_empty().id();
                // ! 开始创建一个异步任务
                // ! 在这个任务代码块中,已经无法获取 World 与 command
                // ! 所以借助 CommandQueue,将实际的 Entity 操作,再次以闭包的方式放入队列,
                // ! 下面的代码中,展示了在闭包(system之外)中,如何获取 World
                let task = thread_pool.spawn(
                    // ! 定义一个异步执行的闭包
                    async move {
                        let duration =
                            Duration::from_secs_f32(rand::thread_rng().gen_range(0.05..5.0));

                        // Pretend this is a time-intensive function. :)
                        // ! 假装这是一个耗时的函数
                        async_std::task::sleep(duration).await;

                        // Such hard work, all done!
                        let transform = Transform::from_xyz(x as f32, y as f32, z as f32);
                        // ! CommandQueue 是一个高密度高效的异步命令队列
                        let mut command_queue = CommandQueue::default();

                        // we use a raw command queue to pass a FnOnce(&mut World) back to be
                        // applied in a deferred manner.
                        command_queue.push(
                            // ! 为command_queue 添加一个闭包,在这个闭包中,使用的是 World 的可写引用
                            move |world: &mut World| {
                                // ! 在闭包中,属于 System 之外,
                                // ! System 之外为了获取资源 (Resource) 需要 SystemState
                                let (box_mesh_handle, box_material_handle) = {
                                    let mut system_state = SystemState::<(
                                        Res<BoxMeshHandle>,
                                        Res<BoxMaterialHandle>,
                                    )>::new(
                                        world
                                    );
                                    let (box_mesh_handle, box_material_handle) =
                                        system_state.get_mut(world);

                                    (box_mesh_handle.clone(), box_material_handle.clone())
                                };

                                // ! 实际创建 entity 的操作
                                world
                                    .entity_mut(entity)
                                    // Add our new `Mesh3d` and `MeshMaterial3d` to our tagged entity
                                    .insert((
                                        Mesh3d(box_mesh_handle),
                                        MeshMaterial3d(box_material_handle),
                                        transform,
                                    ))
                                    // Task is complete, so remove task component from entity
                                    // ! 通过移除 ComputeTransform 组件达成任务完成的目的
                                    .remove::<ComputeTransform>();
                            },
                        );

                        command_queue
                    },
                );

                // Spawn new entity and add our new task as a component
                // ! 首先在 World 中添加的是一个将要进行变化的 entity 任务,
                // ! 最初,它是没有可视信息的
                commands.entity(entity).insert(ComputeTransform(task));
            }
        }
    }
}

/// This system queries for entities that have our Task<Transform> component. It polls the
/// tasks to see if they're complete. If the task is complete it takes the result, adds a
/// new [`Mesh3d`] and [`MeshMaterial3d`] to the entity using the result from the task's work, and
/// removes the task component from the entity.
/// ! 取出所有未可视 (未完成ComputeTransform)
/// ! 以 commands.append 的方式添加执行任务
fn handle_tasks(mut commands: Commands, mut transform_tasks: Query<&mut ComputeTransform>) {
    for mut task in &mut transform_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }
}

/// This system is only used to setup light and camera for the environment
/// ! 设置环境(相机与光源)
fn setup_env(mut commands: Commands) {
    // Used to center camera on spawned cubes
    let offset = if NUM_CUBES % 2 == 0 {
        (NUM_CUBES / 2) as f32 - 0.5
    } else {
        (NUM_CUBES / 2) as f32
    };

    // lights
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 12.0, 15.0)));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(offset, offset, 15.0)
            .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
    ));
}
