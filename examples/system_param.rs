//! This example creates a custom [`SystemParam`] struct that counts the number of players.

use bevy::{ecs::system::SystemParam, prelude::*};

fn main() {
    App::new()
        .insert_resource(PlayerCount(0))
        .add_systems(Startup, spawn)
        .add_systems(Update, count_players)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerCount(usize);

/// The [`SystemParam`] struct can contain any types that can also be included in a
/// system function signature.
///
/// In this example, it includes a query and a mutable resource.
/// 一但定义了 SystemParam,就可以在系统中直接使用,它将会被宏转换成 system 签名中的参数
/// 不需要任何 insert/add 之类的操作
#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'w, 's> PlayerCounter<'w, 's> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

/// Spawn some players to count
fn spawn(mut commands: Commands) {
    commands.spawn(Player);
    commands.spawn(Player);
    commands.spawn(Player);
}

/// The [`SystemParam`] can be used directly in a system argument.
fn count_players(mut counter: PlayerCounter) {
    counter.count();
    // PlayerCounter 定义在本模块中,直接访问 counter 的字段
    println!("{} players in the game", counter.count.0);
}
