//! Shows how to create systems that run every fixed timestep, rather than every tick.

use bevy::prelude::*;

fn main() {
    std::env::set_var("NO_COLOR", "1");
    App::new()
        .add_plugins(DefaultPlugins)
        // this system will run once every update (it should match your screen's refresh rate)
        .add_systems(Update, frame_update)
        // add our system to the fixed timestep schedule
        .add_systems(FixedUpdate, fixed_update)
        // configure our fixed timestep schedule to run twice a second
        // 一秒内执行两次(0.5 * 2)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .run();
}

// 使用 system 关联的 Local 局部存储计算出来的 delta 和 delta_secs() 相差无几
// Local<T> 是一种 system 独有的局部变量.与每个 system 关联,T where Default,
// 会在第一次使用前被初始化
fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    // Default `Time` is `Time<Virtual>` here
    info!(
        "time since last frame_update: {} / {:?}",
        time.elapsed_secs() - *last_time,
        time.delta_secs(),
    );
    *last_time = time.elapsed_secs();
}

fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    // Default `Time`is `Time<Fixed>` here
    info!(
        "time since last fixed_update: {}/{:?}\n",
        time.elapsed_secs() - *last_time,
        time.delta_secs(),
    );

    info!("fixed timestep: {}\n", time.delta_secs());
    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
    // 这里输出的是溢出的 delta
    info!(
        "time accrued toward next fixed_update: {}\n",
        fixed_time.overstep().as_secs_f32()
    );
    *last_time = time.elapsed_secs();
}
