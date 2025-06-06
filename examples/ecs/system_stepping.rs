//! Demonstrate stepping through systems in order of execution.
//!
//! To run this example, you must enable the `bevy_debug_stepping` feature.

use bevy::{ecs::schedule::Stepping, log::LogPlugin, prelude::*};

fn main() {
    let mut app = App::new();

    app
        // to display log messages from Stepping resource
        .add_plugins(LogPlugin::default())
        .add_systems(
            Update,
            (
                update_system_one,
                // establish a dependency here to simplify descriptions below
                update_system_two.after(update_system_one),
                update_system_three.after(update_system_two),
                update_system_four,
            ),
        )
        .add_systems(PreUpdate, pre_update_system);

    // For the simplicity of this example, we directly modify the `Stepping`
    // resource here and run the systems with `App::update()`.  Each call to
    // `App::update()` is the equivalent of a single frame render when using
    // `App::run()`.
    //
    // In a real-world situation, the `Stepping` resource would be modified by
    // a system based on input from the user.  A full demonstration of this can
    // be found in the breakout example.

    // ** 内建的 schedule 没有受到影响,所有 system都被执行
    println!(
        r#"
    Actions: call app.update()
     Result: All systems run normally"#
    );
    app.update();

    // ** 内建的 schedule 没有受到影响,所有 system都被执行
    // ** Stepping 无效,因为没有配置 schedule
    println!(
        r#"
    Actions: Add the Stepping resource then call app.update()
     Result: All systems run normally.  Stepping has no effect unless explicitly
             configured for a Schedule, and Stepping has been enabled."#
    );
    app.insert_resource(Stepping::new());
    app.update();

    // ** 内建的 schedule 受到影响,只有 PreUpdate 的 system 被执行
    println!(
        r#"
    Actions: Add the Update Schedule to Stepping; enable Stepping; call
             app.update()
     Result: Only the systems in PreUpdate run.  When Stepping is enabled,
             systems in the configured schedules will not run unless:
             * Stepping::step_frame() is called
             * Stepping::continue_frame() is called
             * System has been configured to always run"#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.add_schedule(Update).enable();
    app.update();

    // ** 内建的 schdule 受到影响
    // ** PreUpdate 的 system 被执行
    // ** Update 的中的多个 system 被执行一个,因为 setp_frame 只被执行了一次
    // ** 注意, Update Schedule 中的 system 集合,除非明确指定顺序,否则是随机的
    // ** 在本例中,有可能执行的是 update_system_one,也有可能执行的是 update_system_four
    println!(
        r#"
    Actions: call Stepping.step_frame(); call app.update()
     Result: The PreUpdate systems run, and one Update system will run.  In
             Stepping, step means run the next system across all the schedules 
             that have been added to the Stepping resource."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();

    // ** 没有执行 stepping.step_frame 所以,只有 PreUpdate 的 system 被执行
    println!(
        r#"
    Actions: call app.update()
     Result: Only the PreUpdate systems run.  The previous call to
             Stepping::step_frame() only applies for the next call to
             app.update()/the next frame rendered.
    "#
    );
    app.update();

    // ** continue_frame 一次将 Update Schedule 中的所有(剩余的) system 都执行了
    // ** 注意: PreUpdate 的 system 会在每次 app.update() 被执行
    println!(
        r#"
    Actions: call Stepping::continue_frame(); call app.update()
     Result: PreUpdate system will run, and all remaining Update systems will
             run.  Stepping::continue_frame() tells stepping to run all systems
             starting after the last run system until it hits the end of the
             frame, or it encounters a system with a breakpoint set.  In this
             case, we previously performed a step, running one system in Update.
             This continue will cause all remaining systems in Update to run."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    // ** 执行四次,每次执行 PreUpdate 的 system 和 Update 的一个 system
    // ** 注意,再次强调 PreUpdate 每次都执行
    println!(
        r#"
    Actions: call Stepping::step_frame() & app.update() four times in a row
     Result: PreUpdate system runs every time we call app.update(), along with
             one system from the Update schedule each time.  This shows what
             execution would look like to step through an entire frame of 
             systems."#
    );
    for _ in 0..4 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    // ** always_run 会使得 update_system_two 每次都被执行,
    // ** 跳出 set_frame 只运行一个 Update 的 system 的限制
    println!(
        r#"
    Actions: Stepping::always_run(Update, update_system_two); step through all
             systems
     Result: PreUpdate system and update_system_two() will run every time we
             call app.update().  We'll also only need to step three times to
             execute all systems in the frame.  Stepping::always_run() allows
             us to granularly allow systems to run when stepping is enabled."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.always_run(Update, update_system_two);
    for _ in 0..3 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    // ** never_run 屏蔽指定 update_system_two
    println!(
        r#"
    Actions: Stepping::never_run(Update, update_system_two); continue through
             all systems
     Result: All systems except update_system_two() will execute.
             Stepping::never_run() allows us to disable systems while Stepping
             is enabled."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.never_run(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    // ** set_breakpoint 会在 update_system_two 执行时停止(断点一次)
    //** 后续的 update 会单步执行
    println!(
        r#"
    Actions: Stepping::set_breakpoint(Update, update_system_two); continue,
             step, continue
     Result: During the first continue, pre_update_system() and
             update_system_one() will run.  update_system_four() may also run
             as it has no dependency on update_system_two() or
             update_system_three().  Nether update_system_two() nor
             update_system_three() will run in the first app.update() call as
             they form a chained dependency on update_system_one() and run
             in order of one, two, three.  Stepping stops system execution in
             the Update schedule when it encounters the breakpoint for
             update_system_three().
             During the step we run update_system_two() along with the
             pre_update_system().
             During the final continue pre_update_system() and
             update_system_three() run."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.set_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();

    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    // ** 清除所有的断点
    println!(
        r#"
    Actions: Stepping::clear_breakpoint(Update, update_system_two); continue
             through all systems
     Result: All systems will run"#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.clear_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    // 关闭 调试用的 Stepping ,恢复内建的 Schedule
    println!(
        r#"
    Actions: Stepping::disable(); app.update()
     Result: All systems will run.  With Stepping disabled, there's no need to
             call Stepping::step_frame() or Stepping::continue_frame() to run
             systems in the Update schedule."#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.disable();
    app.update();
}

fn pre_update_system() {
    println!("▶ pre_update_system");
}
fn update_system_one() {
    println!("▶ update_system_one");
}
fn update_system_two() {
    println!("▶ update_system_two");
}
fn update_system_three() {
    println!("▶ update_system_three");
}
fn update_system_four() {
    println!("▶ update_system_four");
}

// end
