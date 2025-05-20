use bevy::prelude::*;

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
    pub linear: f32,   // 线性力度
    pub angular: f32,  // 扭力
}

// 安全距离,由武器决定
#[derive(Component, Debug)]
pub struct SafeDistance(pub f32);

#[derive(Default)]
pub struct PerInfo {
    pub shot: f32,
    // 将每秒 shot 转换成一个可用定时器
    pub shot_timer: Option<Timer>,
    pub flux: f32,
    pub damage: f32,
    pub size: f32,
}

#[derive(Default, Debug)]
pub struct PhaseInfo {
    pub speed: f32,
    pub range: f32,
    pub track: f32,
}
