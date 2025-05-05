///! 键(1),观察 cast_ray
///! 键(2),观察 cast_shape
///! 键(空格),清除临时图形
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const START_X: f32 = 1280.0 / 2.0;
const START_Y: f32 = 720.0 / 2.0;

#[derive(Component, Debug)]
struct TempSprite;

fn main() {
    let mut app = App::new();
    let mut rapier_debug = RapierDebugRenderPlugin::default();
    rapier_debug.mode = DebugRenderMode::from_bits(0b111111).unwrap();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        rapier_debug,
    ));

    app.add_systems(Startup, (setup, show_grid));

    app.add_systems(Update, (cast_ray, cast_shape, clear_tmp_sprite));

    app.run();
}

// 创建地板
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // make ground
    make_ground(&mut commands, &mut meshes, &mut materials);
}

// bevy_rapier2d 中,不知道是什么原因,官方文档中的 ReadDefaultRapierContext 并不存在
// 想要获得 raper_context 需要使用 ReadRapierContext.single()
fn cast_ray(
    mut commands: Commands,
    read_rapier: ReadRapierContext,
    keyboard: Res<ButtonInput<KeyCode>>,
) -> Result {
    if !keyboard.just_pressed(KeyCode::Digit1) {
        return Ok(());
    }
    // 第一个测试
    let n = 1.0;

    // (红色)发射源点
    let start_x = -START_X + (1280.0 / 5.0) * n;
    let ray_pos = Vec2::new(start_x, 0.);
    // 在发射源,绘制一个图形
    make_temp_sprite(&mut commands, ray_pos, Color::srgb_u8(128, 0, 0));

    // (蓝色)在向量目标,绘制一个图形
    // 注意: 看到了,并非是基于 ray_pos 的原点进行向量发射,而是基于 (0,0) 确定向量
    // 注意: 但是 Y 轴,依然是一个很重要的数据,它确定了(单位)向量的长度
    let ray_dir = Vec2::new(0., -100.0);
    make_temp_sprite(&mut commands, ray_dir, Color::srgb_u8(0, 0, 128));

    // 撞击时长 ( Time of Impact ),这里可以理解为发射允许的最大时长
    let max_toi = 4.0;

    // 影响发射源被放置在一个物体(Shape)内部时的行为 (未验证,纯释义)
    // true, 说明碰撞发生在起点(因为实心)
    // false, 说明碰撞发生在边缘
    let solid = true;
    let filter = QueryFilter::default();

    // Fixed: 官方文档中不存在的 ReadDefaultRapierContext
    let rapier_context = read_rapier.single()?;

    // 获得 TOI
    if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = ray_pos + ray_dir * toi;
        println!(
            "hit entity:  {:?} as point {:?} / toi: {:?}",
            entity, hit_point, toi
        );
        // (绿色)撞击点
        make_temp_sprite(&mut commands, hit_point, Color::srgb_u8(0, 128, 0));
    }

    // 获得法线
    if let Some((entity, intersection)) =
        rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
    {
        // This is similar to `RapierContext::cast_ray` illustrated above except
        // that it also returns the normal of the collider shape at the hit point.
        let hit_point = intersection.point;
        let hit_normal = intersection.normal;
        println!(
            "Entity {:?} hit at point {} with normal {}",
            entity, hit_point, hit_normal
        );
    }

    // 闭包的方式(同样可以得到法线)
    rapier_context.intersections_with_ray(
        ray_pos,
        ray_dir,
        max_toi,
        solid,
        filter,
        |entity, intersection| {
            // Callback called on each collider hit by the ray.
            let hit_point = intersection.point;
            let hit_normal = intersection.normal;
            println!(
                "Entity {:?} hit at point {} with normal {}",
                entity, hit_point, hit_normal
            );
            true // Return `false` instead if we want to stop searching for other hits.
        },
    );
    Ok(())
}

/* Cast a shape inside of a system. */
fn cast_shape(
    mut commands: Commands,
    read_rapier: ReadRapierContext,
    keyboard: Res<ButtonInput<KeyCode>>,
) -> Result {
    if !keyboard.just_pressed(KeyCode::Digit2) {
        return Ok(());
    }

    // 第二个测试
    let n = 3.0;

    // 发射源点
    let start_x = -START_X + (1280.0 / 5.0) * n;
    let shape = Collider::cuboid(50.0, 50.0);
    let mut shape_pos = Vec2::new(start_x, 0.);
    make_temp_sprite(&mut commands, shape_pos, Color::srgb_u8(128, 0, 0));

    let shape_rot = 0.; // rotation
    //let shape_vel = Vec2::new(0.1, 0.4);
    let shape_vel = Vec2::new(0., -100.);
    make_temp_sprite(&mut commands, shape_vel, Color::srgb_u8(0, 0, 128));

    let mut filter = QueryFilter::default();
    let options = ShapeCastOptions {
        // ! toi
        max_time_of_impact: 1.0,
        // ! 目标距离(但我搞不明白)
        target_distance: 500.0,
        // ! 所谓的穿透,还需要手动调整,不知道意义何在(如果存在暗箱运算,又只能返回一个ShapCastHit)
        stop_at_penetration: false,
        // ! 提供法线等信息
        compute_impact_geometry_on_penetration: true,
    };

    let rapier_context = read_rapier.single()?;
    // 循环两次,测试穿透
    for _ in 0..20 {
        if let Some((entity, hit)) =
            rapier_context.cast_shape(shape_pos, shape_rot, shape_vel, &shape, options, filter)
        {
            // The first collider hit has the entity `entity`. The `hit` is a
            // structure containing details about the hit configuration.
            println!(
                "Hit the entity {:?} with the configuration: {:?}",
                entity, hit
            );

            // hit 的内部属性
            // hit.time_of_impact 撞击时间
            // hit.witness1 撞击点(目标对象)
            // hit.witness2 撞击点(发射对象) 可以看到 约等于 (-5,-5),在第三象限的角被撞击
            // hit.normal1 法线(目标对象)
            // hit.normal2 法线(发射对象)
            make_temp_sprite(
                &mut commands,
                hit.details.unwrap().witness1,
                Color::srgb_u8(0, 128, 0),
            );

            shape_pos += shape_vel * hit.time_of_impact;
            // pub exclude_collider: Option<Entity>,
            // 这里可以看出 exclude_collider 不是一个集合,所以排除的对象只能是一个
            // 在这里,shape的体积过大,所以一直在交替碰撞
            filter = filter.exclude_collider(entity);
            make_temp_sprite(&mut commands, shape_pos, Color::srgb_u8(0, 128, 128));
        } else {
            break;
        }
    }
    Ok(())
}

// 清理示例图形
fn clear_tmp_sprite(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<TempSprite>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// 创建地板
fn make_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // make a ground
    let shape_rectangle = Rectangle::new(1280., 20.);
    let mesh_handle = meshes.add(shape_rectangle);
    let color_handle = materials.add(Color::srgb(0.5, 0.4, 0.3));
    let mut transform = Transform::from_xyz(0., -START_Y + 100.0, 0.);
    transform.rotate_local_z(-0.05);
    let entity = commands
        .spawn((
            RigidBody::Fixed,
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_handle),
            transform,
        ))
        .with_children(|parent| {
            let transform1 = Transform::from_translation(Vec3::new(0., 10., 0.));
            let transform2 = Transform::from_translation(Vec3::new(0., -10., 0.));

            // 用于测试穿透增加两层(1)
            let collider = Collider::cuboid(
                shape_rectangle.half_size.x,
                shape_rectangle.half_size.y / 3.,
            );
            let entity = parent.spawn((collider, transform1)).id();
            println!("(1) ground collider entity: >> {:?} <<", entity);

            // 用于测试穿透增加两层(2)
            let collider = Collider::cuboid(
                shape_rectangle.half_size.x,
                shape_rectangle.half_size.y / 3.,
            );
            let entity = parent.spawn((collider, transform2)).id();
            println!("(2) ground collider entity: >> {:?} <<", entity);
        })
        .id();
    println!("ground entity: >> {:?} <<", entity);
}

fn make_temp_sprite(commands: &mut Commands, pos: Vec2, color: Color) {
    // 在发射源,绘制一个图形
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(10.)),
        Transform::from_translation(pos.extend(0.0)),
        TempSprite,
    ));
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY, // 投影模式
            UVec2::new(16, 9),    // 单元格数量
            Vec2::new(80., 80.),  // 单元格大小
            // Dark gray
            LinearRgba::gray(0.05), // 网格颜色
        )
        .outer_edges();
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
