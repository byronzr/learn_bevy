//! Demonstrates how to propagate events through the hierarchy with observers.

use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use rand::{seq::IteratorRandom, thread_rng, Rng};

fn main() {
    // emacs minibuffer 会有多余控制符,所以屏蔽掉
    std::env::set_var("NO_COLOR", "1");
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        // 生成实体(哥布林)
        .add_systems(Startup, setup)
        // 每间隔200毫秒模拟一次攻击
        .add_systems(
            Update,
            attack_armor.run_if(on_timer(Duration::from_millis(200))),
        )
        // Add a global observer that will emit a line whenever an attack hits an entity.
        // 一个全局的监视器用于显示受到攻击的部位
        // add_observer 全局监视器(最优先传播)
        // observe (局部)监视器
        .add_observer(attack_hits)
        .run();
}

// In this example, we spawn a goblin wearing different pieces of armor. Each piece of armor
// is represented as a child entity, with an `Armor` component.
//
// We're going to model how attack damage can be partially blocked by the goblin's armor using
// event bubbling. Our events will target the armor, and if the armor isn't strong enough to block
// the attack it will continue up and hit the goblin.
// 创建一个哥布林生命值为50
// 哥布林实体承受的伤害来自于被头部/腿部/胸部装备相抵后的剩余量.
// 所以在设计 哥布林这个 Bundle 时,将,三个身体部分设定为 Entity 会更有利于 Query 查询
fn setup(mut commands: Commands) {
    commands
        .spawn((Name::new("Goblin"), HitPoints(50)))
        .observe(take_damage) // 接收 children 传播的 Attack
        .with_children(|parent| {
            parent
                .spawn((Name::new("Helmet"), Armor(5)))
                .observe(block_attack);
            parent
                .spawn((Name::new("Socks"), Armor(10)))
                .observe(block_attack);
            parent
                .spawn((Name::new("Shirt"), Armor(15)))
                .observe(block_attack);
        });
}

// This event represents an attack we want to "bubble" up from the armor to the goblin.
#[derive(Clone, Component)]
struct Attack {
    damage: u16,
}

// We enable propagation by implementing `Event` manually (rather than using a derive) and specifying
// two important pieces of information:
// 手动实现一个可传递的事件 (Event),手动实现
impl Event for Attack {
    // 1. Which component we want to propagate along. In this case, we want to "bubble" (meaning propagate
    //    from child to parent) so we use the `Parent` component for propagation. The component supplied
    //    must implement the `Traversal` trait.
    type Traversal = &'static Parent;
    // 2. We can also choose whether or not this event will propagate by default when triggered. If this is
    //    false, it will only propagate following a call to `Trigger::propagate(true)`.
    // 默认开启 true 后,会就向上传播,直到手动(Trigger::propagate(false))或顶部.
    const AUTO_PROPAGATE: bool = true;
}

/// An entity that can take damage.
/// 实体能承受伤害
/// Deref 与 DerefMut 方便单字段的结构进行快速访问与修改
#[derive(Component, Deref, DerefMut)]
struct HitPoints(u16);

/// For damage to reach the wearer, it must exceed the armor.
/// 伤害到达怪物身上,必须透过装甲
#[derive(Component, Deref)]
struct Armor(u16);

/// A normal bevy system that attacks a piece of the goblin's armor on a timer.
/// 用一个普通的 System 来实现,一个单位时间内哥布林装甲受到攻击
/// 模拟对实体(随机)生成伤害(随机)
fn attack_armor(entities: Query<Entity, With<Armor>>, mut commands: Commands) {
    // 从包含 Armor Component 的实体集中随机获得一个实体,模拟一次攻击伤害
    let mut rng = thread_rng();

    if let Some(target) = entities.iter().choose(&mut rng) {
        // 发送一个 "触发器" 到目标
        let damage = rng.gen_range(1..20);
        commands.trigger_targets(Attack { damage }, target);
        info!("⚔️  Attack for {} damage", damage);
    }
}

/// 仅显示攻击日志
/// 注意: name 实际上是 query, 所以需要 query.get 获得 Name Component
fn attack_hits(trigger: Trigger<Attack>, name: Query<&Name>) {
    if let Ok(name) = name.get(trigger.entity()) {
        info!("Attack hit {}", name);
    } else {
        // 几乎不会在此案例中被打印出来
        info!("let Ok failed.");
    }
}

/// A callback placed on [`Armor`], checking if it absorbed all the [`Attack`] damage.
/// 更新被装甲吸收后的伤害值,并控制是否继续传播
fn block_attack(mut trigger: Trigger<Attack>, armor: Query<(&Armor, &Name)>) {
    let (armor, name) = armor.get(trigger.entity()).unwrap();
    let attack = trigger.event_mut();
    let damage = attack.damage.saturating_sub(**armor);
    if damage > 0 {
        info!("🩸 {} damage passed through {}", damage, name);
        // The attack isn't stopped by the armor. We reduce the damage of the attack, and allow
        // it to continue on to the goblin.
        attack.damage = damage;
    } else {
        info!("🛡️  {} damage blocked by {}", attack.damage, name);
        // Armor stopped the attack, the event stops here.
        // 完全抵消伤害后,并不需要再传播至 take_damage 处理,
        // 所以在这里中止了本次传播
        trigger.propagate(false);
        info!("(propagation halted early)\n");
    }
}

/// A callback on the armor wearer, triggered when a piece of armor is not able to block an attack,
/// or the wearer is attacked directly.
/// 对剩余伤害进行结算
fn take_damage(
    trigger: Trigger<Attack>,
    mut hp: Query<(&mut HitPoints, &Name)>,
    mut commands: Commands,
    mut app_exit: EventWriter<AppExit>,
) {
    let attack = trigger.event();
    let (mut hp, name) = hp.get_mut(trigger.entity()).unwrap();
    **hp = hp.saturating_sub(attack.damage);

    if **hp > 0 {
        info!("{} has {:.1} HP", name, hp.0);
    } else {
        warn!("💀 {} has died a gruesome death", name);
        commands.entity(trigger.entity()).despawn_recursive();
        app_exit.send(AppExit::Success);
    }

    info!("(propagation reached root)\n");
}
