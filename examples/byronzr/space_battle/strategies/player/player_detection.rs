use crate::resources::player::PlayerShipResource;
use crate::utility::track::forward_to;
use crate::{
    components::{
        BaseVelocity,
        ship::{EnemyHull, EnemyProjectPoint, ShipHull, ShipState},
    },
    resources::menu::MainMenu,
};

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

// player_detection
pub fn player_detection(
    mut commands: Commands,
    // 注意: 测试投射点是 EnemyProjectPoint,而不是 EnemyHull,
    // EnemyProjectPoint 是 EnemyHull 的子节点,所以 Transform 是相对于 EnemyHull 的
    // 但是我们需要的是全局坐标,所以我们使用 GlobalTransform
    // Populated 会使 system 在无结果时不会进入,这不是我们想要的,当没有 enemy 时我们希望 ship 回到 中心点
    enemy_query: Query<(Entity, &GlobalTransform), (With<EnemyProjectPoint>, Without<ShipHull>)>,
    // 注意: ShipHull 必须要有一个 Sprite或是Mesh才能有 Transform
    player: Single<(Entity, &mut Transform, &BaseVelocity), (With<ShipHull>, Without<EnemyHull>)>,
    read_context: ReadRapierContext,
    time: Res<Time>,
    mut ship: ResMut<PlayerShipResource>,
    menu: Res<MainMenu>,
) -> Result {
    // 出现敌人后,Populated会使 system 开始运行
    let rapeir_context = read_context.single()?;
    let (player, mut transform, base) = player.into_inner();
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
    let safe_distance = ship.weapon_range;

    if menu.log {
        println!(
            "enemy: {:?} / enemy count: {}",
            point,
            enemy_query.iter().count()
        );
    }

    // 进行跟踪
    ship.engine_flame = forward_to(
        &mut commands,
        &mut transform,
        safe_distance,
        &base,
        player,
        point,
        time.delta_secs(),
        false,
    );

    Ok(())
}
