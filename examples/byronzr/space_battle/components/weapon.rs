use core::f32;

use super::PhaseInfo;
use bevy::prelude::*;
//use bitflags::bitflags;

#[derive(Component, Default, Clone, Debug, Eq, PartialEq)]
pub enum WeaponType {
    Bullet,
    Missile,
    #[default]
    Beam,
}

#[derive(Component, Default)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub entity: Option<Entity>,

    // 弹匣与(填弹)充能时间
    pub capacity: u32,               // 当前弹量
    pub capacity_max: u32,           // 最大容量
    pub capacity_repeat: bool,       // 是否可以重复发射
    pub charge_timer: Option<Timer>, // 充能时间
    pub range: f32,                  // 有效射程
    pub shot_timer: Option<Timer>,   // 每发射击间隔
    pub flux: f32,                   // 每发射击消耗
    pub damage: f32,                 // 每发射击伤害
    pub explode_size: f32,           // 每发射击爆炸范围
    pub fire_angle: f32,             // 发射角度
    pub turn_rate: f32,              // 炮塔可转动角度
    pub phase: Vec<PhaseInfo>,       // 投射物飞行阶段(最多两段)
}

impl Weapon {
    pub fn fire(&mut self) -> bool {
        let Some(shot_timer) = self.shot_timer.as_mut() else {
            return false;
        };

        if self.capacity == 0 || !shot_timer.finished() {
            return false;
        }
        // 开火时间,影响冷却时间
        shot_timer.reset();

        if self.capacity > 0 {
            self.capacity -= 1;
        }
        true
    }
}

impl WeaponType {
    pub fn init(&self, entity: Entity, rate: f32) -> Weapon {
        let mut weapon = Weapon::default();
        weapon.weapon_type = self.clone();
        weapon.entity = Some(entity);
        weapon.turn_rate = rate;
        match *self {
            WeaponType::Bullet => {
                weapon.shot_timer = Some(Timer::from_seconds(2., TimerMode::Once));
                weapon.fire_angle = 0.2;
                weapon.capacity = 0;
                weapon.capacity_max = 10;
                weapon.capacity_repeat = true;
                weapon.charge_timer = Some(Timer::from_seconds(1., TimerMode::Repeating));
                weapon.flux = 5.0;
                weapon.damage = 10.0;
                weapon.explode_size = 3.0;
                weapon.phase.push(PhaseInfo {
                    speed: 100.0,
                    range: 700.0,
                    track: 0.0,
                    lifecycle: Some(Timer::from_seconds(5.0, TimerMode::Once)),
                })
            }
            WeaponType::Missile => {
                weapon.shot_timer = Some(Timer::from_seconds(5., TimerMode::Once));
                weapon.fire_angle = f32::consts::PI; // 全角度
                weapon.capacity = 0;
                weapon.capacity_max = 5;
                weapon.capacity_repeat = false;
                weapon.charge_timer = Some(Timer::from_seconds(5., TimerMode::Repeating));
                weapon.flux = 10.0;
                weapon.damage = 50.0;
                weapon.explode_size = 5.0;
                weapon.phase.push(PhaseInfo {
                    speed: 70.0,
                    range: 1200.0,
                    track: 3.0,
                    lifecycle: Some(Timer::from_seconds(15.0, TimerMode::Once)),
                })
            }
            WeaponType::Beam => {
                weapon.shot_timer = Some(Timer::from_seconds(2., TimerMode::Once));
                weapon.fire_angle = 0.01;
                weapon.capacity = 0;
                weapon.capacity_max = 1;
                weapon.capacity_repeat = true;
                weapon.charge_timer = Some(Timer::from_seconds(3., TimerMode::Repeating));
                weapon.flux = 1.0;
                weapon.damage = 5.0;
                weapon.explode_size = 1.0;
                weapon.phase.push(PhaseInfo {
                    speed: 0.0,
                    range: 550.0,
                    track: 0.0,
                    lifecycle: Some(Timer::from_seconds(5., TimerMode::Once)),
                })
            }
        }
        weapon
    }
}
