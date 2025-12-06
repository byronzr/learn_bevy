//! Illustrates parallel queries with `ParallelIterator`.

use bevy::{ecs::batching::BatchingStrategy, prelude::*};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Component, Deref)]
struct Velocity(Vec2);

/// 创建 entity
fn spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let texture = asset_server.load("branding/icon.png");

    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    // 固定随机数种子,使得每次运行结果一致, 满足了测试中的多样性需求,又能够在调试中复现问题
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    // 创建 128 个 entity,
    // transform 位置默认为 (0, 0, 0),随机加速度
    for _ in 0..128 {
        // rng.random::<f32>() 生成的随机浮点为 0.0 ~ 1.0
        // 为了保证随机散射出去的向量多样性,所以 -0.5,这样就可以在四个象限内随机生成向量,阈值区间就变成了 -0.5 ~ 0.5
        let v = 20.0 * Vec2::new(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5);
        commands.spawn((
            Sprite::from_image(texture.clone()),
            Transform::from_scale(Vec3::splat(1.)),
            Velocity(v),
        ));
    }
}

// Move sprites according to their velocity
fn move_system(mut sprites: Query<(&mut Transform, &Velocity)>) {
    // Compute the new location of each sprite in parallel on the
    // ComputeTaskPool
    //
    // This example is only for demonstrative purposes. Using a
    // ParallelIterator for an inexpensive operation like addition on only 128
    // elements will not typically be faster than just using a normal Iterator.
    // See the ParallelIterator documentation for more information on when
    // to use or not use ParallelIterator over a normal Iterator.
    // 并行迭代器的优势在于大量的物理性多线程运算,在本例子中,屈屈 128 个元素,并行迭代器并不会比普通迭代器快
    // 作为一个示例,但是牛刀杀鸡
    // 向外
    sprites
        .par_iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation += velocity.extend(0.0);
        });
}

// Bounce sprites outside the window
/// 当所有的 entity 超出窗口边界时,反弹回中心,
/// 来回往复
fn bounce_system(window: Single<&Window>, mut sprites: Query<(&Transform, &mut Velocity)>) {
    let width = window.width();
    let height = window.height();
    let left = width / -2.0;
    let right = width / 2.0;
    let bottom = height / -2.0;
    let top = height / 2.0;
    // The default batch size can also be overridden.
    // In this case a batch size of 32 is chosen to limit the overhead of
    // ParallelIterator, since negating a vector is very inexpensive.
    sprites
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(transform, mut v)| {
            if !(left < transform.translation.x
                && transform.translation.x < right
                && bottom < transform.translation.y
                && transform.translation.y < top)
            {
                // For simplicity, just reverse the velocity; don't use realistic bounces
                v.0 = -v.0;
            }
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_system)
        .add_systems(Update, (move_system, bounce_system))
        .run();
}
