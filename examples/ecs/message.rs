//! This example shows how to send, mutate, and receive, events. As well as showing
//! how to you might control system ordering so that events are processed in a specific order.
//! It does this by simulating a damage over time effect that you might find in a game.
//!
//! 该例子由 event.rs 修改而来,主要是将 Event 改为 Message

use bevy::prelude::*;

// In order to send or receive events first you must define them
// This event should be sent when something attempts to deal damage to another entity.
// #[derive(Event, Debug)]
#[derive(Message, Debug)]
struct DealDamage {
    pub amount: i32,
}

// This event should be sent when an entity receives damage.
// #[derive(Event, Debug, Default)]
#[derive(Message, Debug, Default)]
struct DamageReceived;

// This event should be sent when an entity blocks damage with armor.
// #[derive(Event, Debug, Default)]
#[derive(Message, Debug, Default)]
struct ArmorBlockedDamage;

// This resource represents a timer used to determine when to deal damage
// By default it repeats once per second
#[derive(Resource, Deref, DerefMut)]
struct DamageTimer(pub Timer);

impl Default for DamageTimer {
    fn default() -> Self {
        DamageTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

// Next we define systems that send, mutate, and receive events
// This system reads 'DamageTimer', updates it, then sends a 'DealDamage' event
// if the timer has finished.
//
// Events are sent using an 'MessageWriter<T>' by calling 'send' or 'send_default'.
// The 'send_default' method will send the event with the default value if the event
// has a 'Default' implementation.
/// 每1秒发送一个 DealDamage(10)
fn deal_damage_over_time(
    time: Res<Time>,
    mut state: ResMut<DamageTimer>,
    mut events: MessageWriter<DealDamage>,
) {
    if state.tick(time.delta()).is_finished() {
        // Events can be sent with 'send' and constructed just like any other object.
        events.write(DealDamage { amount: 10 });
    }
}

// This system mutates the 'DealDamage' events to apply some armor value
// It also sends an 'ArmorBlockedDamage' event if the value of 'DealDamage' is zero
//
// Events are mutated using an 'EventMutator<T>' by calling 'read'. This returns an iterator
// over all the &mut T that this system has not read yet. Note, you can have multiple
// 'MessageReader', 'MessageWriter', and 'EventMutator' in a given system, as long as the types (T) are different.
/// 取得可修改的 DealDamage(10),将其修改为 DealDamage(9)
/// 发送一个 ArmorBlockedDamage
fn apply_armor_to_damage(
    mut dmg_events: MessageMutator<DealDamage>,
    mut armor_events: MessageWriter<ArmorBlockedDamage>,
) {
    for event in dmg_events.read() {
        event.amount -= 1;
        if event.amount <= 0 {
            // Zero-sized events can also be sent with 'send'
            armor_events.write(ArmorBlockedDamage);
        }
    }
}

// This system reads 'DealDamage' events and sends 'DamageReceived' if the amount is non-zero
//
// Events are read using an 'MessageReader<T>' by calling 'read'. This returns an iterator over all the &T
// that this system has not read yet, and must be 'mut' in order to track which events have been read.
// Again, note you can have multiple 'MessageReader', 'MessageWriter', and 'EventMutator' in a given system,
// as long as the types (T) are different.
/// 读取 DealDamage(9),因为 chain() 的关系
/// 发送一个 DamageReceived,以默认方式 send_default(),因为 DamageReceived 实现了 Default
fn apply_damage_to_health(
    mut dmg_events: MessageReader<DealDamage>,
    mut rcvd_events: MessageWriter<DamageReceived>,
) {
    for event in dmg_events.read() {
        info!("Applying {} damage", event.amount);
        if event.amount > 0 {
            // Events with a 'Default' implementation can be sent with 'send_default'
            rcvd_events.write_default();
        }
    }
}

// Finally these two systems read 'DamageReceived' events.
//
// The first system will play a sound.
// The second system will spawn a particle effect.
//
// As before, events are read using an 'MessageReader' by calling 'read'. This returns an iterator over all the &T
// that this system has not read yet.
/// 第一个 system 读取了 DamageReceived 并不影响第二个 system
/// Event 看似广播了
fn play_damage_received_sound(mut dmg_events: MessageReader<DamageReceived>) {
    for _ in dmg_events.read() {
        info!("Playing a sound.");
    }
}

// Note that both systems receive the same 'DamageReceived' events. Any number of systems can
// receive the same event type.
/// 第二个 system 也能读取到 DamageReceived
fn play_damage_received_particle_effect(mut dmg_events: MessageReader<DamageReceived>) {
    for _ in dmg_events.read() {
        info!("Playing particle effect.");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Events must be added to the app before they can be used
        // using the 'add_event' method
        .add_message::<DealDamage>()
        .add_message::<ArmorBlockedDamage>()
        .add_message::<DamageReceived>()
        .init_resource::<DamageTimer>()
        // As always we must add our systems to the apps schedule.
        // Here we add our systems to the schedule using 'chain()' so that they run in order
        // This ensures that 'apply_armor_to_damage' runs before 'apply_damage_to_health'
        // It also ensures that 'MessageWriters' are used before the associated 'MessageReaders'
        .add_systems(
            Update,
            (
                deal_damage_over_time,
                apply_armor_to_damage,
                apply_damage_to_health,
            )
                .chain(),
        )
        // These two systems are not guaranteed to run in order, nor are they guaranteed to run
        // after the above chain. They may even run in parallel with each other.
        // This means they may have a one frame delay in processing events compared to the above chain
        // In some instances this is fine. In other cases it can be an issue. See the docs for more information
        .add_systems(
            Update,
            (
                play_damage_received_sound,
                play_damage_received_particle_effect,
            ),
        )
        .run();
}
