use bevy::prelude::*;
use bitflags::bitflags;

#[derive(Component, Default, Clone, Debug, Eq, PartialEq)]
pub enum WeaponType {
    Bullet,
    Missile,
    #[default]
    Beam,
}

bitflags! {
    #[derive(Default,Debug)]
    pub struct FireReady: u8 {
        const DISTANCE = 0b00000001;
        const CAPACITY = 0b00000010;
        const FLUX = 0b00000100;
        const REFIRE = 0b00001000;
        const TURNRATE = 0b00010000;
        const ALL = 0b00011111;
        const NONE = 0b00000000;
    }
}

#[derive(Component, Default)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub entity: Option<Entity>,
    pub refire: f32,
    // 将 refire 转换成一个可用定时器
    pub refire_timer: Option<Timer>,
    pub capacity: f32,
    pub per: PerInfo,
    pub phase: Vec<PhaseInfo>,
    pub turn_rate: f32,
    pub fire_ready: FireReady,
}

#[derive(Default)]
pub struct PerInfo {
    pub shot: f32,
    // 将每秒 shot 转换成一个可用定时器
    pub shot_timer: Option<Timer>,
    pub flux: f32,
    pub damage: f32,
    pub size: f32,
}

pub struct PhaseInfo {
    pub speed: f32,
    pub range: f32,
    pub track: f32,
}

impl WeaponType {
    pub fn init(&self, entity: Entity, rate: f32) -> Weapon {
        let mut weapon = Weapon::default();
        weapon.weapon_type = self.clone();
        weapon.entity = Some(entity);
        weapon.turn_rate = rate;
        match *self {
            WeaponType::Bullet => {
                weapon.refire = 2.;
                weapon.capacity = 30.0;
                weapon.per.shot = 10.0;
                weapon.per.flux = 5.0;
                weapon.per.damage = 10.0;
                weapon.per.size = 3.0;
                weapon.phase.push(PhaseInfo {
                    speed: 100.0,
                    range: 200.0,
                    track: 0.0,
                })
            }
            WeaponType::Missile => {
                weapon.refire = 5.;
                weapon.capacity = 10.0;
                weapon.per.shot = 1.0;
                weapon.per.flux = 10.0;
                weapon.per.damage = 50.0;
                weapon.per.size = 5.0;
                weapon.phase.push(PhaseInfo {
                    speed: 70.0,
                    range: 800.0,
                    track: 3.0,
                })
            }
            WeaponType::Beam => {
                weapon.refire = 5.0;
                weapon.capacity = 1.0;
                weapon.per.shot = 1.0;
                weapon.per.flux = 1.0;
                weapon.per.damage = 5.0;
                weapon.per.size = 1.0;
                weapon.phase.push(PhaseInfo {
                    speed: 0.0,
                    range: 150.0,
                    track: 0.0,
                })
            }
        }
        weapon
    }
}
