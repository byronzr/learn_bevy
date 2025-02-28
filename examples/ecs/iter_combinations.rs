//! Shows how to iterate over combinations of query results.

use bevy::{color::palettes::css::ORANGE_RED, math::FloatPow, prelude::*};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 黑底
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, generate_bodies)
        .add_systems(FixedUpdate, (interact_bodies, integrate))
        .add_systems(Update, look_at_star)
        .run();
}

const GRAVITY_CONSTANT: f32 = 0.001;
const NUM_BODIES: usize = 100; // 球体数量

#[derive(Component, Default)]
struct Mass(f32);
#[derive(Component, Default)]
struct Acceleration(Vec3);
#[derive(Component, Default)]
struct LastPos(Vec3);
#[derive(Component)]
struct Star;

#[derive(Bundle, Default)]
struct BodyBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    mass: Mass,
    last_pos: LastPos,
    acceleration: Acceleration,
}

fn generate_bodies(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ico = icosahedron
    // 创建一个基本球体,以方便后续 clone
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());

    // 随机颜色范围
    let color_range = 0.5..1.0;
    //
    let vel_range = -0.5..0.5;

    // We're seeding the PRNG here to make this example deterministic for testing purposes.
    // This isn't strictly required in practical use unless you need your app to be deterministic.
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    for _ in 0..NUM_BODIES {
        // 随机球体半径
        let radius: f32 = rng.gen_range(0.1..0.7);
        // 半径的 3 次方 * 10
        let mass_value = FloatPow::cubed(radius) * 10.;

        // 随机坐标
        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * ops::cbrt(rng.gen_range(0.2f32..1.0))
            * 15.;

        // 创建 entity (很多小球)
        commands.spawn((
            BodyBundle {
                mesh: Mesh3d(mesh.clone()),
                material: MeshMaterial3d(materials.add(Color::srgb(
                    rng.gen_range(color_range.clone()),
                    rng.gen_range(color_range.clone()),
                    rng.gen_range(color_range.clone()),
                ))),
                mass: Mass(mass_value),
                acceleration: Acceleration(Vec3::ZERO),
                last_pos: LastPos(
                    position
                        - Vec3::new(
                            rng.gen_range(vel_range.clone()),
                            rng.gen_range(vel_range.clone()),
                            rng.gen_range(vel_range.clone()),
                        ) * time.timestep().as_secs_f32(),
                ),
            },
            Transform {
                translation: position,
                scale: Vec3::splat(radius),
                ..default()
            },
        ));
    }

    // add bigger "star" body in the center
    // 创建小恒心(橘色)
    let star_radius = 1.;
    commands
        .spawn((
            BodyBundle {
                mesh: Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
                material: MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: ORANGE_RED.into(),
                    emissive: LinearRgba::from(ORANGE_RED) * 2.,
                    ..default()
                })),

                mass: Mass(500.0),
                ..default()
            },
            Transform::from_scale(Vec3::splat(star_radius)),
            Star,
        ))
        // 为小恒心增加点光源(白色)
        .with_child(PointLight {
            color: Color::WHITE,
            range: 100.0,
            radius: star_radius,
            ..default()
        });

    // 创建相机
    // from_xyz(0.0, 10.5, -30.0) 这里设置了摄相机在空间中的位子.
    // looking_at(Vec3::ZERO, Vec3::Y), // 看向 Zero,摄像机顶部朝向Y("取景框"中的Y轴与空间Y轴朝向相同)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.5, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// 组合查询到的所有 Entity 然后,成对组合
// A,B,C => (A,B) / (A,C) / (B,C)
// A,B,C,D ==> (A,B) / (A,C) / (B,C) / (A,D) / (B,D) / (C,D)
// 然后成对的,进行重力影响,该 system 不作任得渲染
fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
        iter.fetch_next()
    {
        let delta = transform2.translation() - transform1.translation();
        let distance_sq: f32 = delta.length_squared();

        let f = GRAVITY_CONSTANT / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

// 将影响结果进行渲染
fn integrate(time: Res<Time>, mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
    let dt_sq = time.delta_secs() * time.delta_secs();
    for (mut acceleration, mut transform, mut last_pos) in &mut query {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)

        let new_pos = transform.translation * 2.0 - last_pos.0 + acceleration.0 * dt_sq;
        acceleration.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}

// 相机会缓慢的 (lerp 插值运算) 朝向 小恒心
fn look_at_star(
    mut camera: Single<&mut Transform, (With<Camera>, Without<Star>)>,
    star: Single<&Transform, With<Star>>,
) {
    let new_rotation = camera
        .looking_at(star.translation, Vec3::Y)
        .rotation
        .lerp(camera.rotation, 0.1);
    camera.rotation = new_rotation;
}
