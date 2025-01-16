use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(".".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        print!("{}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0 = if name.0 == "P" { "Z" } else { "." }.to_string();
    }
}

fn pre_update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0 = if name.0 == "." { "P" } else { "." }.to_string();
    }
}

// 自定义插件
pub struct HelloPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum MoreUpdate {
    First,
    Second,
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, apply_deferred.before(MoreUpdate::Second));
        // app.configure_sets(Update, (MoreUpdate::First, MoreUpdate::Second).chain());
        // app.add_systems(Startup, add_people) // 添加启动系统
        //     .add_systems(Update, greet_people.in_set(MoreUpdate::First))
        //     .add_systems(Update, update_people.in_set(MoreUpdate::Second));
        app.add_systems(Startup, add_people).add_systems(
            Update,
            (greet_people, pre_update_people, update_people).chain(),
        );
    }
}

// 程序入口点
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 默认插件 （UI系统，资源管理，2D/3D渲然）
        .add_plugins(HelloPlugin)
        .run();
}
