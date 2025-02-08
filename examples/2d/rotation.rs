//! Demonstrates rotating entities in 2D using quaternions.

use bevy::{math::ops, prelude::*};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                snap_to_player_system,
                rotate_to_player_system,
            ),
        )
        .run();
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// snap to player ship behavior
#[derive(Component)]
struct SnapToPlayer;

/// rotate to face player ship behavior
#[derive(Component)]
struct RotateToPlayer {
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * `X` axis goes from left to right (`+X` points right)
/// * `Y` axis goes from bottom to top (`+Y` point up)
/// * `Z` axis goes from far to near (`+Z` points towards you, out of the screen)
///
/// The origin is at the center of the screen.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 没有球球
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    // 两球靠下
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    // 两球靠中
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    // 2D orthographic camera
    commands.spawn(Camera2d);

    // 外边留白 1200/4=300, 640/4=160
    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    // player controlled ship
    commands.spawn((
        Sprite::from_image(ship_handle),
        Player {
            movement_speed: 500.0,                  // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        },
    ));

    // enemy that snaps to face the player spawns on the bottom and left
    commands.spawn((
        Sprite::from_image(enemy_a_handle.clone()),
        // ** transform 默认在中心点,
        // ** -x = left
        Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
        // ** 咬住目标
        SnapToPlayer,
    ));
    commands.spawn((
        Sprite::from_image(enemy_a_handle),
        // ** -y = bottom
        Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
        SnapToPlayer,
    ));

    // enemy that rotates to face the player enemy spawns on the top and right
    commands.spawn((
        Sprite::from_image(enemy_b_handle.clone()),
        // ** +x = right
        Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
        //Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
        RotateToPlayer {
            // ** 使用的是弧度值,而不是角度值(反应略慢)
            rotation_speed: f32::to_radians(45.0), // degrees per second
        },
    ));
    commands.spawn((
        Sprite::from_image(enemy_b_handle),
        // ** +y = top
        Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
        RotateToPlayer {
            // ** (反应略快)
            rotation_speed: f32::to_radians(90.0), // degrees per second
        },
    ));
}

/// Demonstrates applying rotation and movement based on keyboard input.
/// ** 左右控制旋转,上是油门,没有刹车与倒车
fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Single<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.into_inner();

    // 一个旋转因子和一个移动因子 默认值
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    // 简单增加旋转因子和移动因子
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }

    // ** 增加一个倒车功能
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        movement_factor -= 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    // ** player 旋转 (Z轴),按弧度值
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());

    // get the ship's forward vector by applying the current rotation to the ships initial facing
    // vector
    // ** 从四元素 rotation 中获取前向向量(实体朝向)
    // ** rotation 是一个四元素,这是为了解决万向节死锁问题等,其它问题
    // ** 在 360 度中,0度是正上方,90度是正右方,180度是正下方,270度是正左方
    // ** 所以使用 Vec3::Y 作为初始向量,因为 Vec3::Y 是正上方（贯例）
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta
    // time
    // ** 速度因子 * 速度 * 时间 = 距离
    // ** 所以速度因子,如果是负数,也不是不可以..
    let movement_distance = movement_factor * ship.movement_speed * time.delta_secs();

    // create the change in translation using the new movement direction and distance
    // ** 当有了方向与距离,就可以计算出位移了
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    // ** 限制移动范围
    // ** BOUNDS 是一个 Vec2,支持除法运算
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

/// Demonstrates snapping the enemy ship to face the player ship immediately.
/// ** 咬住 Player 目标(朝向跟随)
fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_transform: Single<&Transform, With<Player>>,
) {
    // get the player translation in 2D
    // ** Player 坐标(2D)
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        // ** 通过将 目录坐标 - 敌人坐标,得到一个方向向量
        // ** 归一化
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the quaternion to rotate from the initial enemy facing direction to the direction
        // facing the player
        // ** 从目标方向(vec2)求出新的 rotation (四元素)
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        // rotate the enemy to face the player
        // ** 为 enemy 更新 rotation
        enemy_transform.rotation = rotate_to_player;
    }
}

/// Demonstrates rotating an enemy ship to face the player ship at a given rotation speed.
///
/// This method uses the vector dot product to determine if the enemy is facing the player and
/// if not, which way to rotate to face the player. The dot product on two unit length vectors
/// will return a value between -1.0 and +1.0 which tells us the following about the two vectors:
///
/// * If the result is 1.0 the vectors are pointing in the same direction, the angle between them is
///   0 degrees.
/// * If the result is 0.0 the vectors are perpendicular, the angle between them is 90 degrees.
/// * If the result is -1.0 the vectors are parallel but pointing in opposite directions, the angle
///   between them is 180 degrees.
/// * If the result is positive the vectors are pointing in roughly the same direction, the angle
///   between them is greater than 0 and less than 90 degrees.
/// * If the result is negative the vectors are pointing in roughly opposite directions, the angle
///   between them is greater than 90 and less than 180 degrees.
///
/// It is possible to get the angle by taking the arc cosine (`acos`) of the dot product. It is
/// often unnecessary to do this though. Beware than `acos` will return `NaN` if the input is less
/// than -1.0 or greater than 1.0. This can happen even when working with unit vectors due to
/// floating point precision loss, so it pays to clamp your dot product value before calling
/// `acos`.
fn rotate_to_player_system(
    time: Res<Time>,
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        // get the enemy ship forward vector in 2D (already unit length)
        // ** 获取敌人的前向向量（因为不是咬住目标，当前朝向）
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        // ** 获取敌人到玩家的方向向量(最终朝向)
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the dot product between the enemy forward vector and the direction to the player.
        // ** 通过点积（dot product）计算出两个向量的夹角
        // ** 1.0 表示两个向量方向一致
        // ** 0.0 表示两个向量垂直
        // ** -1.0 表示两个向量方向相反
        // ** 如果结果是正数，表示两个向量大致方向相同，夹角在 0 到 90 度之间。
        // ** 如果结果是负数，表示两个向量大致方向相反，夹角在 90 到 180 度之间。
        // ** 两个向量最大角度范围是 180 度
        let forward_dot_player = enemy_forward.dot(to_player);

        // if the dot product is approximately 1.0 then the enemy is already facing the player and
        // we can early out.
        // ** f32::EPSILON 是一个很小的数值,用于比较(浮点数)
        // ** 浮点运算永远钉在耻辱柱上
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // *! 当在2D平面中 Player 与 Enemy 的 Point 基于中心点构成的夹角
        // *! 170度与190度的夹角是一样的(170度),只是方向不一样,10度与350度的夹角是一样的(10度),只是方向不一样
        // *! 所以 forward_dot_player 并不能完全确定旋转方向(左右),如果强制一个固定方向旋转.
        // *! 有可能 10度,需要旋转 350度,这样就会出现问题
        // *! 所以,再次使用 dot product 来确定旋转方向

        // ** 计算是顺时针还是逆时针
        // ** 以敌人的右侧为基准,计算敌人到玩家的方向向量
        // get the right vector of the enemy ship in 2D (already unit length)
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        // get the dot product of the enemy right vector and the direction to the player ship.
        // if the dot product is negative them we need to rotate counter clockwise, if it is
        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
        // with the right vector).
        // ** 1.当点积为负时,表示 Player 相对于Enemy的右侧,处于相反方向.
        // ** 那么,相对于 Enemy 的朝向,Player 在左侧
        // ** 逆时针
        // **
        // ** 2.当点积为正时,表示 Player 相对于Enemy的右侧,处于相同方向.
        // ** 那么,相对于 Enemy 的朝向,Player 在右侧
        // ** 顺时针
        let right_dot_player = enemy_right.dot(to_player);

        // determine the sign of rotation from the right dot player. We need to negate the sign
        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
        // negative is clockwise.
        // ** 为了避免对数值进行判断使用了 f32::copysign 方法,直接取出正负号,并取反
        // ** (逆时针) 在右手坐标系中,+Z
        // ** (顺时针) -Z
        // **
        // ** 结合 dot product 点积,负值 +Z,正值 -Z,
        // ** 正好取反
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        // limit rotation so we don't overshoot the target. We need to convert our dot product to
        // an angle here so we can get an angle of rotation to clamp against.
        // ** 由于浮点数的精度问题,单位向量的点积结果也有可能略微超出[-1.0,1.0],所以使用 clamp 钳制
        // ** 通过点积计算出角度(范围为了保证在在 0~180 内旋转,就需要左右之分了,不可能旋转至 180+)
        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0)); // clamp acos for safety

        // calculate angle of rotation with limit
        // ** 完成旋转差值运算
        let rotation_angle =
            rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);

        // rotate the enemy to face the player
        // ** 转起来
        enemy_transform.rotate_z(rotation_angle);
    }
}
