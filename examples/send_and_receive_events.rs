//! From time to time, you may find that you want to both send and receive an event of the same type in a single system.
//!
//! Of course, this results in an error: the borrows of [`EventWriter`] and [`EventReader`] overlap,
//! if and only if the [`Event`] type is the same.
//! One system parameter borrows the [`Events`] resource mutably, and another system parameter borrows the [`Events`] resource immutably.
//! If Bevy allowed this, this would violate Rust's rules against aliased mutability.
//! In other words, this would be Undefined Behavior (UB)!
//!
//! There are two ways to solve this problem:
//!
//! 1. Use [`ParamSet`] to check out the [`EventWriter`] and [`EventReader`] one at a time.
//! 2. Use a [`Local`] [`EventCursor`] instead of an [`EventReader`], and use [`ResMut`] to access [`Events`].
//!
//! In the first case, you're being careful to only check out only one of the [`EventWriter`] or [`EventReader`] at a time.
//! By "temporally" separating them, you avoid the overlap.
//!
//! In the second case, you only ever have one access to the underlying  [`Events`] resource at a time.
//! But in exchange, you have to manually keep track of which events you've already read.
//!
//! Let's look at an example of each.

use bevy::{core::FrameCount, ecs::event::EventCursor, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_event::<DebugEvent>()
        .add_event::<A>()
        .add_event::<B>()
        .add_systems(Update, read_and_write_different_event_types)
        .add_systems(
            Update,
            (
                send_events,
                debug_events,
                send_and_receive_param_set,
                debug_events,
                send_and_receive_manual_event_reader,
                debug_events,
            )
                .chain(),
        );
    // We're just going to run a few frames, so we can see and understand the output.
    app.update();
    // By running for longer than one frame, we can see that we're caching our cursor in the event queue properly.
    app.update();
}

#[derive(Event)]
struct A;

#[derive(Event)]
struct B;

// This works fine, because the types are different,
// so the borrows of the `EventWriter` and `EventReader` don't overlap.
// Note that these borrowing rules are checked at system initialization time,
// not at compile time, as Bevy uses internal unsafe code to split the `World` into disjoint pieces.
// 合理
// 同一时间只有一个 EventWriter 或 EventReader 被借用,
// 因为 EventWriter/EventReader 是不同的泛型 A/B
fn read_and_write_different_event_types(mut a: EventWriter<A>, mut b: EventReader<B>) {
    for _ in b.read() {}
    a.send(A);
}

/// A dummy event type.
#[derive(Debug, Clone, Event)]
struct DebugEvent {
    resend_from_param_set: bool,
    resend_from_local_event_reader: bool,
    times_sent: u8,
}

/// A system that sends all combinations of events.
/// Res<FrameCount> 桢数累加器(运行期)
/// 发送 DebugEvent 事件的所有组合
fn send_events(mut events: EventWriter<DebugEvent>, frame_count: Res<FrameCount>) {
    println!("Sending events for frame {:?}", frame_count.0);

    events.send(DebugEvent {
        resend_from_param_set: false,
        resend_from_local_event_reader: false,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: true,
        resend_from_local_event_reader: false,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: false,
        resend_from_local_event_reader: true,
        times_sent: 1,
    });
    events.send(DebugEvent {
        resend_from_param_set: true,
        resend_from_local_event_reader: true,
        times_sent: 1,
    });
}

/// A system that prints all events sent since the last time this system ran.
///
/// Note that some events will be printed twice, because they were sent twice.
/// 打印 event 信息
fn debug_events(mut events: EventReader<DebugEvent>) {
    for event in events.read() {
        println!("{event:?}");
    }
}

/// A system that both sends and receives events using [`ParamSet`].
// 利用 ParamSet 将同一类型的 EventWriter 和 EventReader 在一个 scope 中分开
fn send_and_receive_param_set(
    mut param_set: ParamSet<(EventReader<DebugEvent>, EventWriter<DebugEvent>)>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "Sending and receiving events for frame {} with a `ParamSet`",
        frame_count.0
    );

    // We must collect the events to resend, because we can't access the writer while we're iterating over the reader.
    // 事件重发集合,因为我们不能在迭代读取器时访问写入器
    let mut events_to_resend = Vec::new();

    // This is p0, as the first parameter in the `ParamSet` is the reader.
    // 迭代一个事件读取器
    for event in param_set.p0().read() {
        // 如果事件中的 resend_from_param_set 为 true,则将事件添加到 events_to_resend 中
        if event.resend_from_param_set {
            events_to_resend.push(event.clone());
        }
    }

    // This is p1, as the second parameter in the `ParamSet` is the writer.
    // 从 events_to_resend 逐个对 event.times_sent 进行递增
    // 然后 通过 param_set.p1() 发送事件
    for mut event in events_to_resend {
        event.times_sent += 1;
        param_set.p1().send(event);
    }
}

/// A system that both sends and receives events using a [`Local`] [`EventCursor`].
fn send_and_receive_manual_event_reader(
    // The `Local` `SystemParam` stores state inside the system itself, rather than in the world.
    // `EventCursor<T>` is the internal state of `EventReader<T>`, which tracks which events have been seen.
    // EventCursor , 可以理解为 是 EventReader 的读取 "游标/指针"
    // 可能 Local<T> 进行了类似 Arc<Mutex<T>> 的封装,
    // 这样,就符合 RUST 同一 scope 中不能同时存在可变引用和不可变引用的规则
    mut local_event_reader: Local<EventCursor<DebugEvent>>,
    // We can access the `Events` resource mutably, allowing us to both read and write its contents.
    // 一个可以被修改的 Events 资源
    // 注意: 这是一个资源,不是一个迭代器(EventReader),它的最主要作用是完成 send
    mut events: ResMut<Events<DebugEvent>>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "Sending and receiving events for frame {} with a `Local<EventCursor>",
        frame_count.0
    );

    // We must collect the events to resend, because we can't mutate events while we're iterating over the events.
    let mut events_to_resend = Vec::new();

    for event in local_event_reader.read(&events) {
        if event.resend_from_local_event_reader {
            // For simplicity, we're cloning the event.
            // In this case, since we have mutable access to the `Events` resource,
            // we could also just mutate the event in-place,
            // or drain the event queue into our `events_to_resend` vector.
            events_to_resend.push(event.clone());
        }
    }

    for mut event in events_to_resend {
        event.times_sent += 1;
        events.send(event);
    }
}
