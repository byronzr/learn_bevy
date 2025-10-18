//! Illustrates how `Timer`s can be used both as resources and components.

use bevy::{log::info, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Countdown>() // 添加一个计时器资源
        .add_systems(Startup, setup) // spawn 一个 PrintOnCompletionTimer 唯一组件的实体
        .add_systems(Update, (countdown, print_when_completed)) // countdown 用于 Countdown 逻辑,print_when_completed 用于 PrintOnCompletionTimer 逻辑
        .run();
}

// 虽然在此处是只是定义了一个 Component
// 但在 spawn 中,没有其它的 Component 组合
// 可以"简单"理解为,这就是一个实体
#[derive(Component, Deref, DerefMut)]
struct PrintOnCompletionTimer(Timer);

// 一个倒数计时器
// percent_trigger 为重复使用的 timer ,用于触发节点进度
// main_timer 一个单次计时器,定义总时长
#[derive(Resource)]
struct Countdown {
    percent_trigger: Timer,
    main_timer: Timer,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            percent_trigger: Timer::from_seconds(4.0, TimerMode::Repeating),
            main_timer: Timer::from_seconds(20.0, TimerMode::Once),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

fn setup(mut commands: Commands) {
    // Add an entity to the world with a timer
    commands.spawn(PrintOnCompletionTimer(Timer::from_seconds(
        5.0,
        TimerMode::Once,
    )));
}

/// This system ticks the `Timer` on the entity with the `PrintOnCompletionTimer`
/// component using bevy's `Time` resource to get the delta between each update.
// 判断 entity 的 timer 是否 finished
// timer.tick 会推进 timer 的状态,所以需要 mut
fn print_when_completed(time: Res<Time>, mut query: Query<&mut PrintOnCompletionTimer>) {
    for mut timer in &mut query {
        if timer.tick(time.delta()).just_finished() {
            info!("Entity timer just finished");
        }
    }
}

/// This system controls ticking the timer within the countdown resource and
/// handling its state.
// 判断 Resource timer 的状态
// 同样, Countdown 中的 Timer 需要更新状态,所以使用 ResMut
fn countdown(time: Res<Time>, mut countdown: ResMut<Countdown>) {
    countdown.main_timer.tick(time.delta());

    // The API encourages this kind of timer state checking (if you're only checking for one value)
    // Additionally, `finished()` would accomplish the same thing as `just_finished` due to the
    // timer being repeating, however this makes more sense visually.
    if countdown.percent_trigger.tick(time.delta()).just_finished() {
        if !countdown.main_timer.finished() {
            // Print the percent complete the main timer is.
            info!(
                "Timer is {:0.0}% complete!",
                countdown.main_timer.fraction() * 100.0
            );
        } else {
            // The timer has finished so we pause the percent output timer
            countdown.percent_trigger.pause();
            info!("Paused percent trigger timer");
        }
    }
}
