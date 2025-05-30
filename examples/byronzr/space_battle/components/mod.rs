#![allow(dead_code)]
use bevy::prelude::*;

pub mod effects;
pub mod ship;
pub mod weapon;

#[derive(Component, Debug, Default)]
pub struct Projectile {
    pub damage: f32,
    pub size: f32,
    pub phase1: PhaseInfo,
    pub phase2: PhaseInfo,
}

// 基础速度与扭力
#[derive(Component, Debug)]
pub struct BaseVelocity {
    pub speed: f32,
    pub torque: f32,
    pub braking: Braking,
}

// 基础制动系数
#[derive(Debug)]
pub struct Braking {
    pub distance: f32, // 制动距离
    pub speed: f32,    // 线性力度
    pub torque: f32,   // 扭力
}

impl Default for Braking {
    fn default() -> Self {
        Self {
            distance: 5.0,
            speed: 1.0,
            torque: 0.1,
        }
    }
}

#[derive(Default, Debug)]
pub struct PhaseInfo {
    pub speed: f32,
    pub range: f32,
    pub track: f32,
    pub lifecycle: Option<Timer>,
}
