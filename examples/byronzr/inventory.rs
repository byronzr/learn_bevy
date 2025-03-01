#![allow(dead_code)]

use std::fmt::Debug;

use bevy::{
    color::palettes::css, prelude::*, sprite::Anchor, utils::HashMap, winit::WinitSettings,
};
use rand::{rng, seq::IndexedRandom};

// 定义左上角的坐标,基于UI坐标系
const OFFSET: Vec2 = Vec2::new(200., 200.);

// 物品行与列
const ROWS: usize = 12;
const COLS: usize = 16;

// 物品栏格子大小
// (实际格子大小 = SLOT_SIZE - SLOT_GAP),保证物品与其对齐时不需要额外的偏移
const SLOT_SIZE: f32 = 32.;

// 格子间隙
const SLOT_GAP: f32 = 1.;

// 保证格子层在物品之下
const SLOT_LAYER: f32 = 1.;
const ITEM_LAYER: f32 = 2.;

/// * 用于记录物品栏的格子索引,以左上角为起点(0,0)
#[derive(Component, Debug)]
pub struct SlotIndex(pub (usize, usize));

// * 插件使用状态,
// ! 不直接使用 commands 的原因, 因为 observe 不允许请求 commands 参数,
// ! 所以需要通过插件的方式传递,让另一个触发器来完成颜色修改
#[derive(Component, Debug)]
pub struct SlotUsed(pub bool);

#[derive(Component, Debug, Clone)]
pub struct ItemSlots {
    pub path: String,
    pub angle: i32, // 0 度 Y轴开始,每次顺时针旋转 90 度
    pub size: (usize, usize),
    pub slots: Vec<(usize, usize)>, // size 换算出来的矩形
    pub unset: Vec<(usize, usize)>, // 使用排除则为异形格子
}

impl ItemSlots {
    pub fn new(size: (usize, usize), path: &str) -> Self {
        let mut slots = vec![];
        for x in 0..size.0 {
            for y in 0..size.1 {
                slots.push((x, y));
            }
        }
        Self {
            path: path.to_string(),
            angle: 0,
            size,
            slots,
            unset: vec![],
        }
    }

    pub fn rotate(&mut self) {
        self.angle = (self.angle + 90) % 360;
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        self.unset.push((x, y));
    }
}

/// 库存清单的占用信息
#[derive(Resource, Debug, Default)]
struct InventorySlotMap(pub HashMap<(usize, usize), bool>);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(WinitSettings::desktop_app());
    app.init_resource::<InventorySlotMap>();

    // @ 展示了 UI 层遮挡 Sprite 使物品堆叠不可视,所以完全放弃使用 UI 进行物品栏的绘制
    // app.add_systems(Startup, setup_node_ui);

    // @ 使用 Sprite 进行物品栏的绘制
    app.add_systems(Startup, setup_sprite_ui);

    // @ 使用 gizmos 进行调试
    // @ gizmos 需要多次运行才能可视,所以要写在 Update 系统中
    app.add_systems(Update, (draw_gizmos, generate_random_item));
    app.run();
}

/// 创建一个随机物品
fn generate_random_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (it, mut bg, mut bc, children) in &mut interaction_query {
        match *it {
            Interaction::Pressed => {
                bg.0 = Color::from(css::DARK_SALMON);
                bc.0 = Color::WHITE;
                let items = vec![
                    ItemSlots::new((1, 1), "items/paper.png"),
                    ItemSlots::new((1, 2), "items/knife.png"),
                    ItemSlots::new((6, 2), "items/AK47.png"),
                ];
                let mut rng = rng();

                let Some(item) = items.choose(&mut rng).cloned() else {
                    return;
                };
                let handle = asset_server.load(item.path.as_str());

                commands
                    .spawn((
                        Sprite::from_image(handle),
                        item,
                        Transform::from_xyz(0., 0., ITEM_LAYER),
                    ))
                    //.observe(observe_slot::<Pointer<Click>>())
                    //.observe(observe_slot::<Pointer<DragDrop>>()) // ! 在这里,DragDrop 方法不会产生效果,因为 slot 没有对应的触发器
                    .observe(observe_item::<Pointer<DragEnd>>())
                    .observe(observe_item::<Pointer<Drag>>());
            }
            Interaction::Hovered => {
                bg.0 = Color::from(css::DARK_BLUE.with_alpha(0.8));
                bc.0 = Color::WHITE;
            }
            _ => {}
        }
    }
}

/// slot 响应拖拽事件
/// ! &ItemSlots 确保了只有物品才会触发
fn observe_slot(
    trigger: Trigger<Pointer<DragEnter>>,
    mut query: Query<(Entity, &mut Sprite, &mut Transform, &ItemSlots)>,
    mut solt_info: ResMut<InventorySlotMap>,
) {
    // @ 在 DragEnter 中,dragged 是拾起实体,target 是叠加实体
    for (query_entity, mut sprite, mut transform, is_item) in &mut query {
        // ! 仅处理物品, 因为一但出现物品,如果不校验格子,则会出现格子可被拖拽的信息
        if trigger.dragged != query_entity {
            continue;
        }
        info!(
            "DragEnter: target => {:?} dragged => {:?} ev.entity => {:?},query_entity: {:?},hit_data: {:?}",
            trigger.target,
            trigger.dragged,
            trigger.entity(),
            query_entity,
            //trigger.hit,
            true,
        );
        break;
    }
}

/// 支持拖拽事件
fn observe_item<E: Debug + Reflect + Clone>() -> impl Fn(
    Trigger<E>,
    Query<(Entity, &mut Sprite, &mut Transform, &ItemSlots), Without<SlotIndex>>,
    ResMut<InventorySlotMap>,
) {
    move |ev, mut query, mut solt_info| {
        for (query_entity, mut sprite, mut transform, is_item) in &mut query {
            // ! 出现多个物体时,只处理当前拖拽的物体
            if ev.entity() != query_entity {
                continue;
            }

            let reflect = ev.event().try_as_reflect().unwrap();

            if let Some(trigger) = reflect.downcast_ref::<Pointer<DragEnd>>() {
                info!("DragEnd: {:?}", trigger.pointer_location);
                transform.translation.z = ITEM_LAYER; // ! 结束时,我们需要物品栏保持在最上层
                break;
            }

            if let Some(trigger) = reflect.downcast_ref::<Pointer<Drag>>() {
                let delta = trigger.delta * Vec2::new(1., -1.);
                transform.translation += delta.extend(0.);
                transform.translation.z = SLOT_LAYER; // ! 移动时,我们需要触发 DragEnter 事件,需要与 Slot 在同一层(z轴)
                break;
            }
        }
    }
}

/// 绘制物品栏
fn setup_sprite_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_query: Single<&Window>,
    mut solt_map: ResMut<InventorySlotMap>,
) {
    // 创建一个 UI 相机
    commands.spawn(Camera2d);

    let size = win_query.physical_size();
    let size = Vec2::new(size.x as f32, size.y as f32);
    info!("window size: {:?}", size);

    // window top_left
    let window_top_left = Vec2::new(-size.x / 2., size.y / 2.);
    info!("window top_left: {:?}", window_top_left);

    // 选算出大小
    let width = COLS as f32 * SLOT_SIZE;
    let height = ROWS as f32 * SLOT_SIZE;
    info!("inventory size: (width: {}, height: {})", width, height);

    // 整体居中偏移量
    // 左上角 -x,+y
    let start_offset = window_top_left + (OFFSET * Vec2::new(1., -1.));
    info!("center offset: {:?}", start_offset);

    for col in 0..COLS {
        for row in 0..ROWS {
            let transform = Transform::from_translation(Vec3::new(
                start_offset.x + col as f32 * SLOT_SIZE,
                start_offset.y - row as f32 * SLOT_SIZE,
                SLOT_LAYER,
            ));
            // TODO: 还差半格 slot_size 未处理
            let _ = commands
                .spawn((
                    // Sprite {
                    //     color: Color::from(css::GREY.with_alpha(0.1)),
                    //     custom_size: Some(Vec2::splat(SLOT_SIZE - SLOT_GAP)),
                    //     anchor: Anchor::TopLeft,
                    //     ..default()
                    // },
                    Sprite::from_color(
                        Color::from(css::GREY.with_alpha(0.1)),
                        Vec2::splat(SLOT_SIZE - SLOT_GAP),
                    ),
                    transform,
                    SlotIndex((col, row)),
                    // Anchor::Center,
                ))
                .with_child((
                    Text2d::new(format!("{},{}", col, row)),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                ))
                //.observe(observe_slot::<Pointer<Drag>>())
                //.observe(observe_slot::<Pointer<DragDrop>>())
                .observe(observe_slot)
                //.observe(observe_slot::<Pointer<DragEnd>>())
                .id();
        }
    }

    // 放一个生成按钮
    // ! 未来可以使用 children![] 宏,简化嵌套
    commands
        .spawn(Node {
            // * 不是不可以使用 UI 层与 Sprite 配合,
            // * 最主要的原因是 UI 层需要布局时,需要最外层的 Node 以 Val::Percent(100.) 的方式占用整个窗口
            // * 一但占用了整个窗口,鼠标的事件就无法穿透 UI 层,
            // * 在这里,使用了 UI 层,但蔽屏了宽高的要求,UI容器以很小的方式悬浮在了左上角(打开试试)
            // width: Val::Percent(100.),
            // height: Val::Percent(100.),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(5.0)),
                        // width: Val::Px(150.0),
                        // height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(1.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(5.0)),
                    BorderColor(Color::WHITE),
                    BackgroundColor(Color::from(css::DARK_ORANGE)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Generate a random item"),
                        TextFont {
                            font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// 支持拖拽事件
// fn observe_slot<E: Debug + Reflect + Clone>()
// -> impl Fn(Trigger<E>, Query<(&mut Sprite, &mut Transform), With<SlotIndex>>) {
//     move |ev, mut query| {
//         info!("observe_slot: {:?}", ev);
//         let reflect = ev.event().try_as_reflect().unwrap();

//         if let Some(trigger) = reflect.downcast_ref::<Pointer<DragEnd>>() {
//             info!("DragEnd: {:?}", trigger.pointer_location);
//         }
//     }
// }

/// ! 调试用的 gizmos
fn draw_gizmos(mut gizmos: Gizmos, win_query: Single<&Window>) {
    // @ 通过 Window 始终获得最新的 size 有可能中途 resing
    let size = win_query.physical_size();
    let size = Vec2::new(size.x as f32, size.y as f32);
    gizmos.rect_2d(Isometry2d::IDENTITY, size, css::YELLOW_GREEN);
}

/// ! 这段代码展示了 UI Node 无法与 Sprite 配合实现物品栏的绘制
fn setup_node_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 创建一个 UI 相机
    commands.spawn((Camera2d, IsDefaultUiCamera));

    // 加载物品图片
    let h = asset_server.load("items/paper.png");

    //
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(100.),
                    aspect_ratio: Some(1.),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));
        });

    //
    commands.spawn((Sprite::from_image(h),));
}
