//! This example illustrates how to use [`States`] for high-level app control flow.
//! States are a powerful but intuitive tool for controlling which logic runs when.
//! You can have multiple independent states, and the [`OnEnter`] and [`OnExit`] schedules
//! can be used to great effect to ensure that you handle setup and teardown appropriately.
//!
//! In this case, we're transitioning from a `Menu` state to an `InGame` state.
//!
//! 需要 feature 运行
//! cargo run --example states --features bevy_dev_tools
//! 模拟一个游戏状态机,包括菜单和游戏状态

use bevy::{dev_tools::states::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // AppState 实现了 Default 特性，所以可以使用 init_state 方法
        .init_state::<AppState>() // Alternatively we could use .insert_state(AppState::Menu)
        // 简单的添加一个 camera2d
        .add_systems(Startup, setup)
        // This system runs when we enter `AppState::Menu`, during the `StateTransition` schedule.
        // All systems from the exit schedule of the state we're leaving are run first,
        // and then all systems from the enter schedule of the state we're entering are run second.
        // 1.当进入状态 AppState::Menu，则运行 setup_menu 系统
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        // By contrast, update systems are stored in the `Update` schedule. They simply
        // check the value of the `State<T>` resource to see if they should run each frame.
        // 2. 如果当前状态是 AppState::Menu，则运行 menu 系统
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        // 3.如果退出 AppState::Menu，则运行 cleanup_menu 系统
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        // 4.如果进入 AppState::InGame，则运行 setup_game 系统
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(
            Update,
            // 5.如果当前状态是 AppState::InGame，则运行 movement 系统
            (movement, change_color).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, log_transitions::<AppState>)
        .run();
}

/// States 的定义
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
/// 创建菜单布局
fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(
            // 首先创建一个布局
            // 注意: Node 主要影响的其子实体的布局
            (
                Node {
                    // center button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // 所以这里的 Text 并不会居中
                // Text::new("text"),
            ),
        )
        // 在(相对)布局中创建一个按钮
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    // 按钮(内部)的布局
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // Text::default(),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                // 在(相对)按钮中创建一个文本
                .with_children(|parent| {
                    parent.spawn((
                        // 会在父级中寻找 Text,如果找不到,没有效果,
                        // 因为是在父级中的 Text 进行操作
                        // 所以也不会受父级 Node 的影响
                        // TextSpan::new("span1"),
                        Text::new("Play"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        })
        .id();

    commands.insert_resource(MenuData { button_entity });
}

/// 影响菜单中 Button 的事件
fn menu(
    // mut state: ResMut<State<AppState>>, // 不使用该资源的目的是因为它会立即触发状态转换
    mut next_state: ResMut<NextState<AppState>>, // 使用 NextState 来触发状态转换,保证在下一桢时生效
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            // 按下,推进状态
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// 简单的清理掉 menu 的按钮
fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}

/// 简单添加一个 Sprite 作为控制对象
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
}

/// 简单的响应键盘事件,控制 Sprite 移动
fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    const SPEED: f32 = 100.0;
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_secs();
        }
    }
}

/// 简单的变换颜色
fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in &mut query {
        let new_color = LinearRgba {
            blue: ops::sin(time.elapsed_secs() * 0.5) + 2.0,
            ..LinearRgba::from(sprite.color)
        };

        sprite.color = new_color.into();
    }
}
