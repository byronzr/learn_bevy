//! Demonstrates how the `AlignItems` and `JustifyContent` properties can be composed to layout text.
use bevy::prelude::*;

const ALIGN_ITEMS_COLOR: Color = Color::srgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::srgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(12.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Flex Layout Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2d);
    commands
        .spawn((
            // ** 最外层的 Node,整体布局
            Node {
                // fill the entire window
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column, // * 子元素竖排
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN, // ! 行间距
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|builder| {
            // spawn the key
            // ** 两个 Text 标题节点
            // ** 本身受 Column 影响
            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Row, // * 子元素横排
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(
                        builder,
                        font.clone(),
                        ALIGN_ITEMS_COLOR,
                        UiRect::right(MARGIN),
                        "AlignItems",
                    );
                    spawn_nested_text_bundle(
                        builder,
                        font.clone(),
                        JUSTIFY_CONTENT_COLOR,
                        UiRect::default(),
                        "JustifyContent",
                    );
                });

            // ** AlignItems 和 JustifyContent 的组合
            // ** 本身受 Column 影响
            builder
                .spawn(Node {
                    width: Val::Percent(100.), // 会自动填满剩余空间
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column, // * 子元素排列方向,向下排列
                    row_gap: MARGIN,
                    ..default()
                })
                .with_children(|builder| {
                    // spawn one child node for each combination of `AlignItems` and `JustifyContent`
                    // ** 子元素受 Column 影响,所以一次为一行
                    // ** 一行有 6 个子元素
                    // ** 每个 cell 会自适应大小
                    let justifications = [
                        JustifyContent::FlexStart,
                        JustifyContent::Center,
                        JustifyContent::FlexEnd,
                        JustifyContent::SpaceEvenly,
                        JustifyContent::SpaceAround,
                        JustifyContent::SpaceBetween,
                    ];
                    // ** 一共 5 行
                    let alignments = [
                        AlignItems::Baseline,
                        AlignItems::FlexStart,
                        AlignItems::Center,
                        AlignItems::FlexEnd,
                        AlignItems::Stretch,
                    ];

                    // ** 从行开始循环,即从 AlignItems 开始
                    for align_items in alignments {
                        builder
                            .spawn(
                                // ** 行的式样
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    column_gap: MARGIN,
                                    ..Default::default()
                                },
                            )
                            .with_children(|builder| {
                                for justify_content in justifications {
                                    // ** 列的式样 (cell)
                                    spawn_child_node(
                                        builder,
                                        font.clone(),
                                        align_items, // ! 控制子元素在交叉轴上的对齐方式
                                        justify_content, // ! 控制子元素在主轴上的对齐方式 (取决父级FlexDirection)
                                    );
                                }
                            });
                    }
                });
        });
}

fn spawn_child_node(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    align_items: AlignItems,
    justify_content: JustifyContent,
) {
    builder
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items,
                justify_content,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .with_children(|builder| {
            let labels = [
                (format!("{align_items:?}"), ALIGN_ITEMS_COLOR, 0.),
                (format!("{justify_content:?}"), JUSTIFY_CONTENT_COLOR, 3.),
            ];
            for (text, color, top_margin) in labels {
                // We nest the text within a parent node because margins and padding can't be directly applied to text nodes currently.
                // ** 文本受父级 Node的影响,已经确定了布局方式
                spawn_nested_text_bundle(
                    builder,
                    font.clone(),
                    color,
                    UiRect::top(Val::Px(top_margin)),
                    &text,
                );
            }
        });
}

fn spawn_nested_text_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
            BackgroundColor(background_color),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                TextFont { font, ..default() },
                TextColor::BLACK,
            ));
        });
}
