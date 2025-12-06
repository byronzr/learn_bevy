//! Showcases the [`RelativeCursorPosition`] component, used to check the position of the cursor relative to a UI node.

use bevy::{
    prelude::*, render::camera::Viewport, ui::RelativeCursorPosition, winit::WinitSettings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, relative_cursor_position_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            // Cursor position will take the viewport offset into account
            // ! 在这里对视窗进行了偏移,
            viewport: Some(Viewport {
                // ! 定位到 (200, 100) 的位置
                physical_position: [200, 100].into(),
                // ! 设置视窗大小为 600x600,如果超出视窗大小,将不会渲染
                // ! 地于多视窗与UI隔离有帮助
                physical_size: [600, 600].into(),
                ..default()
            }),
            ..default()
        },
    ));

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // 矩形
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(250.),
                        margin: UiRect::bottom(Val::Px(15.)),
                        ..default()
                    },
                    // ! 在官方的例子中,这个颜色是白色,srgb 接收的 rgb 参数阈值是 0-1,
                    // ! 这里都超过了 1. 相当于都是 1.0
                    BackgroundColor(Color::srgb(235., 35., 12.)),
                    //BackgroundColor(Color::BLACK),
                ))
                // ! 为这个 Entity 添加一个 RelativeCursorPosition 组件
                // ! 此时,相对位子是原点是矩形的左上角,这符合直觉
                .insert(RelativeCursorPosition::default());

            parent.spawn((
                Text::new("(0.0, 0.0)"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// This systems polls the relative cursor position and displays its value in a text component.
fn relative_cursor_position_system(
    relative_cursor_position: Single<&RelativeCursorPosition>, // * 相对光标位置
    output_query: Single<(&mut Text, &mut TextColor)>,         // * 文本输出
) {
    let (mut output, mut text_color) = output_query.into_inner();

    // ! option<Vec2> 有值时,说明鼠标相对原点的位置(矩形左上角)
    **output = if let Some(relative_cursor_position) = relative_cursor_position.normalized {
        format!(
            "({:.1}, {:.1})",
            relative_cursor_position.x, relative_cursor_position.y
        )
    }
    // ! 无值时,说明超出了边界
    else {
        "unknown".to_string()
    };

    text_color.0 = if relative_cursor_position.mouse_over() {
        Color::srgb(0.1, 0.9, 0.1)
    } else {
        Color::srgb(0.9, 0.1, 0.1)
    };
}
