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
    println!("- PreUpdate: real time is printed"); // Time<Real>
    println!("- FixedUpdate: fixed time step time is printed, may be run zero or multiple times"); // Time<Fixed>
    println!("- Update: virtual game time is printed"); // Time<Virtual>
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
        // 只有用户输入才会响应
        match line.unwrap().as_str() {
            "" => {
                // 非控制字符,执行所有 schedule(PreUpdate/Update/FixedUpdate/......)
                app.update();
            }
            // 将 Time<Virtual> 的相对速度设置为 2x
            "f" => {
                println!("FAST: setting relative speed to 2x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(2.0);
            }
            // 将 Time<Virtual> 的相对速度设置为 1x
            "n" => {
                println!("NORMAL: setting relative speed to 1x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(1.0);
            }
            // 将 Time<Virtual> 的相对速度设置为 0.5x
            "s" => {
                println!("SLOW: setting relative speed to 0.5x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(0.5);
            }
            // 暂停 Time<Virtual>
            "p" => {
                println!("PAUSE: pausing virtual clock");
                app.world_mut().resource_mut::<Time<Virtual>>().pause();
            }
            // 恢复 Time<Virtual>
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

// Time<Real> 为真实时间,需要显示申明获取
fn print_real_time(time: Res<Time<Real>>) {
    println!(
        "PreUpdate: this is real time clock, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

// 所有 Fixed 前缀的 Schedule 的 Time 都是 Time<Fixed>
fn print_fixed_time(time: Res<Time>) {
    println!(
        "FixedUpdate: this is generic time clock inside fixed, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

// 除了 Fixed 前缀的 Schedule 的 Time 都是 Time<Virtual>
fn print_time(time: Res<Time>) {
    println!(
        "Update: this is generic time clock, delta is {:?} and elapsed is {:?}",
        time.delta(),
        time.elapsed()
    );
}

fn main() {
    // Time<Real> 不会受到影响

    App::new()
        .add_plugins(MinimalPlugins)
        // 钳制最大 delta 为 5 秒,超过 5 秒的 delta 会被截断
        // Virtual 设置了主循环(单桢)时间的步长,会影响到 Time<Fixed>
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
        // 受主循环(单桢)时间的步长影响,
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .add_systems(PreUpdate, print_real_time)
        .add_systems(FixedUpdate, print_fixed_time)
        .add_systems(Update, print_time)
        // 手动设置一个call function 只会调用一次
        // 为了持续交互,runner 中实现了一个循环
        .set_runner(runner)
        .run();
}
