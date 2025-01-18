//! An example that illustrates how Time is handled in ECS.

use bevy::{app::AppExit, prelude::*};

use std::{
    io::{self, BufRead},
    time::Duration,
};

fn banner() {
    println!("This example is meant to intuitively demonstrate how Time works in Bevy.");
    println!();
    println!("Time will be printed in three different schedules in the app:");
    println!("- PreUpdate: real time is printed");
    println!("- FixedUpdate: fixed time step time is printed, may be run zero or multiple times");
    println!("- Update: virtual game time is printed");
    println!();
    println!("Max delta time is set to 5 seconds. Fixed timestep is set to 1 second.");
    println!();
}

fn help() {
    println!("The app reads commands line-by-line from standard input.");
    println!();
    println!("Commands:");
    println!("  empty line: Run app.update() once on the Bevy App");
    println!("  q: Quit the app.");
    println!("  f: Set speed to fast, 2x");
    println!("  n: Set speed to normal, 1x");
    println!("  s: Set speed to slow, 0.5x");
    println!("  p: Pause");
    println!("  u: Unpause");
}

fn runner(mut app: App) -> AppExit {
    banner();
    help();

    let stdin = io::stdin();
    // 阻塞等待用户输入
    for line in stdin.lock().lines() {
        if let Err(err) = line {
            println!("read err: {err:#}");
            break;
        }
        match line.unwrap().as_str() {
            "" => {
                // 非控制字符,执行所有 schedule(PreUpdate/Update/FixedUpdate/......)
                app.update();
            }
            // 在游戏世界里,除了 Time<Virtual> 有变更调整的必要, Time<Real> 与 Time<Fixed> 并不需要频繁修改
            "f" => {
                println!("FAST: setting relative speed to 2x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(2.0);
            }
            "n" => {
                println!("NORMAL: setting relative speed to 1x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(1.0);
            }
            "s" => {
                println!("SLOW: setting relative speed to 0.5x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(0.5);
            }
            "p" => {
                println!("PAUSE: pausing virtual clock");
                app.world_mut().resource_mut::<Time<Virtual>>().pause();
            }
            "u" => {
                println!("UNPAUSE: resuming virtual clock");
                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
            }
            "q" => {
                println!("QUITTING!");
                break;
            }
            _ => {
                help();
            }
        }
    }

    AppExit::Success
}

// Time<Real>
// time.delta() 显示上一桢到此刻的 delta 的值,
// time.elapsed() app,实际运行时间
// 该方法每 app.update() 只会运行一次,由 PreUpdate 调度
fn print_real_time(time: Res<Time<Real>>) {
    println!(
        "PreUpdate: this is real time clock, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

// Time == Time<Fixed>
// 因为 FixedUpdate 在调用它
// 该方法受 Virtual 钳制最大 delta 与 自身 Duration 影响
// 在调用 app.update 时,会计算至上一桢至此的 delta 值,如果超过 Virtual 设置的 max_delta ,则 delta 值为 max_delta
// 按自身 Duration 长度进行分切,FixedUpdate,会按分切个数调用该方法
fn print_fixed_time(time: Res<Time>) {
    println!(
        "FixedUpdate: this is generic time clock inside fixed, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

// Time == Time<Virtual>
// 未指定 Res 时,默认使用 Virtual 会让游戏速度更灵活
// 该方法每 app.update() 只会运行一次
// time.delta() 最大只会是 5s
// time.elapsed() 的增量累积也会受到 time.delta() 影响
fn print_time(time: Res<Time>) {
    println!(
        "Update: this is generic time clock, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // 钳制最大 delta 为 5 秒,这会影响到 FixedUpdate
        // 在本例中,如果超过 5 秒后再输入, FixedUpdate 只会调用 5 次,因为 FixedUpdate 每秒只会调用一次
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
        // FixedUpdate 每秒只会调用一次,但 Update 默认情况下是 1/60 次,也就是桢率
        // Update 更多的时候服务视觉需求
        // FixedUpdate 更多时间服务于游戏逻辑
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .add_systems(PreUpdate, print_real_time)
        .add_systems(FixedUpdate, print_fixed_time)
        .add_systems(Update, print_time)
        // 手动设置一个call function 只会调用一次
        // 为了持续交互,runner 中实现了一个循环
        .set_runner(runner)
        .run();
}
