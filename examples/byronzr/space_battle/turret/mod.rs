use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use projectile::Emit;

use crate::player::{ShipHull, ShipPart, ShipResource};

pub mod projectile;
pub mod weapon;
use weapon::{FireReady, Weapon, WeaponType};

#[derive(Resource)]
pub struct WeaponResource {
    pub weapon: Vec<Weapon>,
    pub fire_type: WeaponType,
    pub text: String,
}

impl WeaponResource {
    pub fn set_type(&mut self, ty: WeaponType) {
        self.fire_type = ty;
        self.weapon.iter_mut().for_each(|w| {
            w.refire_timer = None;
            w.per.shot_timer = None;
        });
    }
    pub fn available_weapons(&mut self) -> Vec<&mut Weapon> {
        self.weapon
            .iter_mut()
            .filter(|w| w.weapon_type == self.fire_type)
            .collect::<Vec<_>>()
    }
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeaponResource {
            weapon: vec![],
            fire_type: WeaponType::default(),
            text: format!("Weapon Type: {:?}", WeaponType::default(),),
        });
        app.add_systems(Update, detect_enemy);
        app.add_observer(projectile::emit_observer);
    }
}

// 炮塔挂载点自行计算射程与角度
fn detect_enemy(
    mut commands: Commands,
    mut res_weapon: ResMut<WeaponResource>,
    ship: Res<ShipResource>,
    // 注意: 这里我们使到的是 GlobalTransform,因为 ShipPart 是以 Children 方式与 ShipHull 绑定的
    query: Populated<(Entity, &GlobalTransform), With<ShipPart>>,
    hull: Single<&Transform, With<ShipHull>>,
    read_rapier: ReadRapierContext,
    mut gizmos: Gizmos,
    time: Res<Time>,
) -> Result {
    // 当前可用武器列表
    let available_weapons = res_weapon.available_weapons();

    // hull position
    let hull_pos = hull.into_inner().translation.truncate();

    let rapier_context = read_rapier.single()?;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::ALL, Group::GROUP_19));

    // 开始检测
    for weapon in available_weapons {
        let Some(entity) = weapon.entity else {
            continue;
        };
        let Ok((_entity, transform)) = query.get(entity) else {
            continue;
        };
        // 炮塔挂载点
        let mount_pos = transform.translation().truncate();
        // 通过 hull_pos 与 挂载点,得到发射向量
        let mount_direction = (mount_pos - hull_pos).normalize();
        // 是否存在敌人
        if let Some((_enemy_entity, projection)) =
            rapier_context.project_point(mount_pos, true, filter)
        {
            // 先画个箭头
            gizmos.arrow_2d(mount_pos, projection.point, Color::srgb_u8(0, 255, 255));

            // 挂载点与目标向量
            let enemy_direction = projection.point - mount_pos;

            // 计算 direction 向量与 hull_direction 向量的夹角
            let angle = mount_direction.angle_to(enemy_direction);
            if angle < weapon.turn_rate {
                weapon.fire_ready |= FireReady::TURNRATE;
            }

            // 计算射程
            let distance = mount_pos.distance(projection.point);
            if let Some(range) = weapon.phase.get(0).and_then(|p| Some(p.range)) {
                if distance < range {
                    weapon.fire_ready |= FireReady::DISTANCE;
                } else {
                    // NOTE: x & !mask 清除某位(始终为0)
                    weapon.fire_ready &= !FireReady::DISTANCE;
                    continue;
                }
            }

            // TODO: flux
            weapon.fire_ready |= FireReady::FLUX;

            // TODO: capacity
            weapon.fire_ready |= FireReady::CAPACITY;

            // refire
            if weapon.refire_timer.is_none() {
                weapon.refire_timer = Some(Timer::new(
                    Duration::from_secs_f32(weapon.refire),
                    TimerMode::Repeating,
                ));
                continue;
            }

            if let Some(timer) = weapon.refire_timer.as_mut() {
                timer.tick(time.delta());
                if timer.just_finished() {
                    weapon.fire_ready |= FireReady::REFIRE;
                } else {
                    weapon.fire_ready &= !FireReady::REFIRE;
                }
            }

            if weapon.fire_ready.bits() == FireReady::ALL.bits() {
                //println!("refire: {:?}", weapon.fire_ready);
                commands.trigger(Emit {
                    direction: enemy_direction.normalize(),
                    start_position: mount_pos,
                })
            }
        }
    }

    Ok(())
}
