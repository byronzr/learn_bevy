use crate::resources::player::PlayerShipResource;
use crate::utility::track;
use crate::{
    components::{
        BaseVelocity, SafeDistance,
        ship::{EnemyHull, EnemyProjectPoint, ShipHull, ShipState},
    },
    resources::menu::MainMenu,
};

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use core::f32;

// track target
pub fn track_target(
    commands: &mut Commands,
    transform: &mut Transform,
    safe_distance: f32,
    base: &BaseVelocity,
    player: Entity,
    target: Vec2,
    delta_time: f32,
    log: bool,
) -> bool {
    let mut flame = false;
    // 计算角度,NONE 表示无需旋转
    let rotate = track::rotaion_to(target, transform);
    if let Some((angle, clockwise)) = rotate {
        // 计算差值
        let rotation_value = clockwise * (base.torque * delta_time).min(angle);
        // 按差值旋转
        transform.rotate_z(rotation_value);
    }

    // 从飞船到目标的向量 (目标-飞船)
    let (forward, distance) = track::forward(target, transform);
    let max_step = distance - safe_distance;
    // 计算速度差值
    let velocity = (distance * base.speed * delta_time).min(max_step);
    // 速度为负数(pulse反向)
    if velocity < f32::EPSILON {
        return flame;
    }

    //if rotate.is_some() {
    // 施加脉冲
    flame = true;
    //}

    // 当转向时移速会变慢
    let force = forward
        * velocity
        * if rotate.is_some() || distance < safe_distance {
            0.5
        } else {
            1.0
        };
    // 施加驱动力(脉冲)
    commands.entity(player).insert(ExternalImpulse {
        impulse: force,
        torque_impulse: base.torque,
    });
    if log {
        println!(
            "player: {:?}, target: {:?}, distance: {}, force: {}",
            player, target, distance, force
        );
    }
    flame
}

// player_detection
pub fn player_detection(
    mut commands: Commands,
    // 注意: 测试投射点是 EnemyProjectPoint,而不是 EnemyHull,
    // EnemyProjectPoint 是 EnemyHull 的子节点,所以 Transform 是相对于 EnemyHull 的
    // 但是我们需要的是全局坐标,所以我们使用 GlobalTransform
    // Populated 会使 system 在无结果时不会进入,这不是我们想要的,当没有 enemy 时我们希望 ship 回到 中心点
    enemy_query: Query<(Entity, &GlobalTransform), (With<EnemyProjectPoint>, Without<ShipHull>)>,
    // 注意: ShipHull 必须要有一个 Sprite或是Mesh才能有 Transform
    player: Single<
        (Entity, &mut Transform, &BaseVelocity, &SafeDistance),
        (With<ShipHull>, Without<EnemyHull>),
    >,
    read_context: ReadRapierContext,
    time: Res<Time>,
    mut ship: ResMut<PlayerShipResource>,
    menu: Res<MainMenu>,
) -> Result {
    // 出现敌人后,Populated会使 system 开始运行
    let rapeir_context = read_context.single()?;
    let (player, mut transform, base, safe) = player.into_inner();
    // 注意: 投射查询的是 EnemyProjectPoint,而不是 EnemyHull
    let filter = QueryFilter::default().groups(CollisionGroups::new(Group::ALL, Group::GROUP_18));
    let ship_pos = transform.translation.xy();
    // 敌人消失后,才会进行新的测试,防止策略摇摆
    let point = if let Some(enemy) = ship.target_enmey {
        if let Ok((_, transform)) = enemy_query.get(enemy) {
            transform.translation().xy()
        } else {
            // 能够运行到这里,说明还有敌人存在,只是未进行投射,所以,我们将 target 设为 None
            // 下次就进行投射了
            ship.target_enmey = None;
            Vec2::ZERO
        }
    } else {
        if let Some((enemy, projection)) = rapeir_context.project_point(ship_pos, true, filter) {
            ship.state = ShipState::Moving;
            ship.target_enmey = Some(enemy);
            projection.point
        } else {
            // 完全没有敌人了
            Vec2::ZERO
        }
    };

    if menu.log {
        println!("enemy: {:?}", point);
    }

    // 进行跟踪
    ship.engine_flame = track_target(
        &mut commands,
        &mut transform,
        safe.0,
        &base,
        player,
        point,
        time.delta_secs(),
        menu.log,
    );

    Ok(())
}
