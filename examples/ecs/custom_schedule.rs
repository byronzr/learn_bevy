//! Demonstrates how to add custom schedules that run in Bevy's `Main` schedule, ordered relative to Bevy's built-in
//! schedules such as `Update` or `Last`.

use bevy::{
    app::MainScheduleOrder,
    ecs::schedule::{ExecutorKind, ScheduleLabel},
    prelude::*,
};

// 一个单线程的 Schedule
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct SingleThreadedUpdate;

// 一个(多线程) Schedule
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct CustomStartup;

fn main() {
    let mut app = App::new();

    // Create a new [`Schedule`]. For demonstration purposes, we configure it to use a single threaded executor so that
    // systems in this schedule are never run in parallel. However, this is not a requirement for custom schedules in
    // general.
    // 将自定义 Schedule 实例化,并且将执行器修改为单线程(独占)
    let mut custom_update_schedule = Schedule::new(SingleThreadedUpdate);
    custom_update_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

    // Adding the schedule to the app does not automatically run the schedule. This merely registers the schedule so
    // that systems can look it up using the `Schedules` resource.
    // 先将 schedule 添加至资源,并不会自动运行
    // schedule 实例的作用在后续操作中就不大了,后续的操作中,可以直接使用 SingleThreadedUpdate 作为标识
    app.add_schedule(custom_update_schedule);

    // Bevy `App`s have a `main_schedule_label` field that configures which schedule is run by the App's `runner`.
    // By default, this is `Main`. The `Main` schedule is responsible for running Bevy's main schedules such as
    // `Update`, `Startup` or `Last`.
    //
    // We can configure the `Main` schedule to run our custom update schedule relative to the existing ones by modifying
    // the `MainScheduleOrder` resource.
    //
    // Note that we modify `MainScheduleOrder` directly in `main` and not in a startup system. The reason for this is
    // that the `MainScheduleOrder` cannot be modified from systems that are run as part of the `Main` schedule.
    // App 默认的 Runner 会调用默认的 main_schedule_label 字段,称其为 Main Schedule,运行时,Main Schedule 会响应
    // Update / Startup ... 已经预设好的 Schedule.
    // 可以通过 MainSchduleOrder 标识,读取这个 Main Schdule,对其进行配置.将自定义 Schedule 插入
    // 需要注意的是,对于 MainScheduleOrder 的配置,只能在 main 主函数中进行,不能将这个过程 "系统化"
    // 原因是,system 的执行本身依赖于 MainScheduleOrder
    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_after(Update, SingleThreadedUpdate);

    // Adding a custom startup schedule works similarly, but needs to use `insert_startup_after`
    // instead of `insert_after`.
    // 再次添加一个自定义 schedule ,因为保持其多线程的默认设置,
    // 不需要从实例中进行配置,所以,直接将其初始化后转移所有权加入资源
    app.add_schedule(Schedule::new(CustomStartup));

    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_startup_after(PreStartup, CustomStartup);

    // 对于 Startup 是不允许使用 insert_after 的
    // main_schedule_order.insert_after(PreStartup, CustomStartup);

    app.add_systems(SingleThreadedUpdate, single_threaded_update_system)
        .add_systems(CustomStartup, custom_startup_system)
        .add_systems(PreStartup, pre_startup_system)
        .add_systems(Startup, startup_system)
        .add_systems(First, first_system)
        .add_systems(Update, update_system)
        .add_systems(Last, last_system)
        .run();
}

fn pre_startup_system() {
    println!("Pre Startup");
}

fn startup_system() {
    println!("Startup");
}

fn custom_startup_system() {
    println!("Custom Startup");
}

fn first_system() {
    println!("First");
}

fn update_system() {
    println!("Update");
}

fn single_threaded_update_system() {
    println!("Single Threaded Update");
}

fn last_system(mut w: EventWriter<AppExit>) {
    println!("Last");
    w.send(AppExit::Success);
}
