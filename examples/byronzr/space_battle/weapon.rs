use bevy::prelude::*;
use bevy_ecs::entity;

use crate::player::{ShipPart, ShipResource};

#[derive(Component, Default, Clone, Debug, Eq, PartialEq)]
pub enum WeaponType {
    #[default]
    Bullet,
    Missile,
    Beam,
    Hamer,
}

#[derive(Component, Default)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub entity: Option<Entity>,
    pub refire: f32,
    pub capacity: f32,
    pub per: PerInfo,
    pub phase: Vec<PhaseInfo>,
}

#[derive(Default)]
pub struct PerInfo {
    pub shot: f32,
    pub flux: f32,
    pub damage: f32,
    pub size: f32,
}

pub struct Projectile {
    pub damage: f32,
    pub size: f32,
    pub phase: PhaseInfo,
}

pub struct PhaseInfo {
    pub speed: f32,
    pub range: f32,
    pub track: f32,
}

impl WeaponType {
    pub fn init(&self, entity: Entity) -> Weapon {
        let mut weapon = Weapon::default();
        weapon.weapon_type = self.clone();
        weapon.entity = Some(entity);
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
                weapon.refire = 0.0;
                weapon.capacity = 0.0;
                weapon.per.shot = 1.0;
                weapon.per.flux = 1.0;
                weapon.per.damage = 5.0;
                weapon.per.size = 1.0;
                weapon.phase.push(PhaseInfo {
                    speed: 0.0,
                    range: 50.0,
                    track: 0.0,
                })
            }
            WeaponType::Hamer => {
                weapon.refire = 0.0;
                weapon.capacity = 0.0;
                weapon.per.shot = 1.0;
                weapon.per.flux = 1.0;
                weapon.per.damage = 5.0;
                weapon.per.size = 1.0;
                weapon.phase.push(PhaseInfo {
                    speed: 300.0,
                    range: 250.0,
                    track: 0.0,
                })
            }
        }
        weapon
    }
}

#[derive(Resource)]
pub struct WeaponResource {
    pub weapon: Vec<Weapon>,
    pub fire_type: WeaponType,
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeaponResource {
            weapon: vec![],
            fire_type: WeaponType::default(),
        });
        app.add_systems(Update, detect_enemy);
    }
}

fn detect_enemy(
    mut commands: Commands,
    weapon: Res<WeaponResource>,
    ship: Res<ShipResource>,
    query: Populated<(Entity, &Transform), With<ShipPart>>,
) {
    let available_weapons = weapon
        .weapon
        .iter()
        .filter(|w| w.weapon_type == weapon.fire_type)
        .collect::<Vec<_>>();
    for weapon in available_weapons {
        let Some(entity) = weapon.entity else {
            continue;
        };
        let Ok((_entity, transform)) = query.get(entity) else {
            continue;
        };
    }
}
