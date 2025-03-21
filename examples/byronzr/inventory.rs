#![allow(dead_code)]

use std::{fmt::Debug, sync::LazyLock};

use bevy::{
    color::palettes::css, prelude::*, sprite::Anchor, utils::hashbrown::HashSet,
    winit::WinitSettings,
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

// 颜色
static SLOT_UNUSE: LazyLock<Color> = LazyLock::new(|| Color::from(css::GREY.with_alpha(0.1)));
static SLOT_USED: LazyLock<Color> = LazyLock::new(|| Color::from(css::DARK_GREEN.with_alpha(0.1)));

/// 用于记录物品栏的格子索引,以左上角为起点(0,0)
#[derive(Component, Debug)]
pub struct SlotIndex(pub (usize, usize));

// 物品栏格子的占用信息
#[derive(Component, Debug, Clone, Default)]
pub struct ItemSlots {
    pub path: String,
    pub angle: i32, // 0 度 Y轴开始,每次顺时针旋转 90 度
    pub size: (usize, usize),
    pub slot_metrix: HashSet<(usize, usize)>, // size 换算出来的矩形
    pub slot_used: HashSet<(usize, usize)>,   // 当前占用的格子(映射库存清单中的格子)
    pub slot_position: Vec2,                  // 物品栏中的位置
}

impl ItemSlots {
    pub fn new(size: (usize, usize), path: &str) -> Self {
        let mut instance = Self {
            path: path.to_string(),
            size,
            ..default()
        };
        instance.calc_slots();
        instance
    }

    pub fn rotate(&mut self) {
        self.angle = (self.angle + 90) % 180;
        std::mem::swap(&mut self.size.0, &mut self.size.1);
        self.calc_slots();
    }

    fn calc_slots(&mut self) {
        let mut slots = HashSet::new();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                slots.insert((x, y));
            }
        }
        self.slot_metrix = slots;
    }
}

/// 库存清单格子的占用信息
#[derive(Resource, Debug, Default)]
struct InventorySlotStatus(pub HashSet<(usize, usize)>);

/// 当前拾取的 Entity
/// 因为 Trigger 的 observe 事件,没有 commands, 所以保存到 Resource 中
#[derive(Resource, Debug, Default)]
struct PickedEntity(pub Option<Entity>);

/// 未摆放标记
#[derive(Component, Debug)]
struct UnSlotted;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(WinitSettings::desktop_app());
    app.init_resource::<InventorySlotStatus>();
    app.init_resource::<PickedEntity>();

    // 使用 Sprite 进行物品栏的绘制
    app.add_systems(Startup, setup_sprite_ui);

    // 使用 gizmos 进行调试
    // gizmos 需要多次运行才能可视,所以要写在 Update 系统中
    app.add_systems(
        Update,
        (
            draw_gizmos,
            generate_random_item,
            auto_slot,
            picked_item_rotate,
        ),
    );

    app.add_systems(PostUpdate, update_slot_status);
    app.run();
}

/// 实时更新 slot 的状态(颜色)
fn update_slot_status(
    slot_info: Res<InventorySlotStatus>,
    mut query: Query<(&mut Sprite, &SlotIndex)>,
) {
    for (mut sprite, slot_index) in &mut query {
        let ref key = slot_index.0;
        // let used = slot_info.total_used.iter().find(|k| *k == key);
        let color = if slot_info.0.contains(key) {
            *SLOT_USED
        } else {
            *SLOT_UNUSE
        };
        sprite.color = color;
    }
}

/// 对新建物品进行第一次排放
fn auto_slot(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut ItemSlots, &UnSlotted)>,
    slot_query: Query<(&SlotIndex, &Transform), Without<ItemSlots>>,
    mut slot_info: ResMut<InventorySlotStatus>,
) {
    // 构建一个左上角开始的索引
    let mut idx_iter = vec![];
    for row in 0..ROWS {
        for col in 0..COLS {
            idx_iter.push((col, row));
        }
    }

    for (query_entity, mut transform, mut is_item, _) in &mut query {
        // 寻找可用格子(按左上角开始寻找)
        for idx in idx_iter.iter() {
            // 如果有格子被占用,寻找下一个
            if slot_info.0.contains(idx) {
                continue;
            }
            'slot_iter: for (slot_index, slot_position) in slot_query.iter() {
                // 如果不是指定的 slot 则跳过
                if slot_index.0 != *idx {
                    continue;
                }

                // 记录将要被占用的格子
                let mut slots = HashSet::new();
                // 根据物品的大小,计算出占用的格子
                for (x, y) in &is_item.slot_metrix {
                    let x = x + slot_index.0.0;
                    let y = y + slot_index.0.1;
                    // 如果有格子被占用,中断退出
                    if slot_info.0.contains(&(x, y)) {
                        break 'slot_iter;
                    }
                    // 超出边界,中断退出
                    if x >= COLS || y >= ROWS {
                        break 'slot_iter;
                    }
                    slots.insert((x, y));
                }

                // 记录占用的格子
                slot_info.0 = slot_info.0.union(&slots).cloned().collect();
                is_item.slot_used = slots;
                is_item.slot_position = slot_position.translation.truncate();

                // 直接完成位移
                transform.translation = slot_position.translation;
                transform.translation.z = ITEM_LAYER;

                // 移除未排放标记
                commands.entity(query_entity).remove::<UnSlotted>();

                return;
            }
        }
        // 如果找到不存储的格子,删除物品
        //commands.entity(query_entity).despawn();
        commands.entity(query_entity).try_despawn_recursive();
    }
}

/// 创建一个随机物品
fn generate_random_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (it, mut bg, mut bc) in &mut interaction_query {
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
                        //Sprite::from_image(handle),
                        Sprite {
                            image: handle,
                            anchor: Anchor::TopLeft,
                            ..default()
                        },
                        item.clone(),
                        Transform::from_xyz(0., 0., ITEM_LAYER),
                        UnSlotted, // 标记未排放
                    ))
                    .observe(observe_item::<Pointer<DragStart>>()) // ! 处理点击事件开始时,将 item 进行位移到鼠标
                    .observe(observe_item::<Pointer<DragEnd>>()) // ! 处理点击事件结束时,将 item 进行位移到物品栏对齐
                    .observe(observe_item::<Pointer<Drag>>()); // ! 移动事件
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
fn observe_slot(
    trigger: Trigger<Pointer<DragEnter>>,
    mut items_query: Query<&mut ItemSlots>,
    slot_query: Query<(&SlotIndex, &Transform), Without<ItemSlots>>,
    mut slot_info: ResMut<InventorySlotStatus>,
) {
    // 当前物品
    let Ok(mut is_item) = items_query.get_mut(trigger.dragged) else {
        return;
    };

    // 取出当前格子
    let Ok((slot_index, slot_position)) = slot_query.get(trigger.target) else {
        return;
    };

    // 缓存当前的占用信息,如果物品无法正确放置,则用于恢复
    let prev = slot_info.0.clone();
    // 移动时释放物品栏的占用
    slot_info.0 = slot_info
        .0
        .difference(&is_item.slot_used)
        .cloned()
        .collect();

    // 根据索引,查看是否有格子被占用
    if slot_info.0.contains(&slot_index.0) {
        slot_info.0 = prev;
        return;
    }

    // 记录将要被占用的格子
    let mut slots = HashSet::new();

    // 根据物品的大小,计算出占用的格子
    for (x, y) in &is_item.slot_metrix {
        let x = x + slot_index.0.0;
        let y = y + slot_index.0.1;

        // 如果有格子被占用,中断退出
        if slot_info.0.contains(&(x, y)) {
            slot_info.0 = prev;
            return;
        }
        // 超出边界,中断退出
        if x >= COLS || y >= ROWS {
            slot_info.0 = prev;
            return;
        }
        slots.insert((x, y));
    }

    // 记录占用的格子
    slot_info.0 = slot_info.0.union(&slots).cloned().collect();
    is_item.slot_used = slots;
    // 记录要对齐的格子
    is_item.slot_position = slot_position.translation.truncate();
}

fn picked_item_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut items_query: Query<(&mut Sprite, &mut ItemSlots, &mut Transform)>,
    picked: Res<PickedEntity>,
) {
    // 处理物品旋转
    if input.just_pressed(KeyCode::KeyR) {
        let Some(picked_entity) = picked.0 else {
            return;
        };
        let Ok((mut sprite, mut item, mut transform)) = items_query.get_mut(picked_entity) else {
            return;
        };
        item.rotate();
        sprite.anchor = Anchor::Center;
        transform.rotation = Quat::from_rotation_z((item.angle as f32).to_radians());
        sprite.anchor = match item.angle {
            0 => Anchor::TopLeft,
            _ => Anchor::TopRight,
        };
        return;
    }
}

/// item 响应拖拽事件,
/// 事件是在有鼠标状态变化时触发的,
/// 所以如果需要实时接收键盘输入,需要将当前拾取的物品的 Entity 保存,在 DragEnd 时释放
fn observe_item<E: Debug + Reflect + Clone>()
-> impl Fn(Trigger<E>, Query<(&mut Transform, &mut ItemSlots), Without<SlotIndex>>, ResMut<PickedEntity>)
{
    move |ev, mut query, mut picked| {
        let Ok((mut transform, is_item)) = query.get_mut(ev.entity()) else {
            return;
        };

        let reflect = ev.event().try_as_reflect().unwrap();

        // 第一次点击时,将物品(左上角)移动到鼠标位置
        if let Some(trigger) = reflect.downcast_ref::<Pointer<DragStart>>() {
            let Some(start) = trigger.event.hit.position else {
                return;
            };
            transform.translation = start;
            return;
        }

        // 结束时,我们需要物品栏保持在最上层
        if let Some(_trigger) = reflect.downcast_ref::<Pointer<DragEnd>>() {
            transform.translation = is_item.slot_position.extend(ITEM_LAYER);
            picked.0 = None;
            return;
        }

        // 移动时,我们需要触发 DragEnter 事件,需要与 Slot 在同一层(z轴)
        // Y轴的坐标是反的,所以需要转换
        if let Some(trigger) = reflect.downcast_ref::<Pointer<Drag>>() {
            let delta = trigger.delta * Vec2::new(1., -1.);
            transform.translation += delta.extend(0.);
            transform.translation.z = SLOT_LAYER;
            picked.0 = Some(ev.entity());
            return;
        }
    }
}

/// 绘制物品栏
fn setup_sprite_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_query: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    // 创建一个 UI 相机
    commands.spawn(Camera2d);

    // 获取物理屏幕大小(主要用于纹理相关)
    //let size = win_query.physical_size();

    // 窗体逻辑大小(用于UI布局)
    let size = win_query.size();
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
            let custom_size = Vec2::splat(SLOT_SIZE - SLOT_GAP);
            let _ = commands
                .spawn((
                    // * 也可以使用 Mesh2 ,但 Sprite 相对更简单
                    Sprite {
                        color: Color::from(css::GREY.with_alpha(0.1)),
                        custom_size: Some(custom_size),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform,
                    SlotIndex((col, row)),
                    // Anchor::Center,
                ))
                // 显示坐标索引
                .with_child((
                    Text2d::new(format!("{},{}", col, row)),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                    // 位移到格子中心,Sprite 的 Anchor 为 TopLeft,所以视觉上不是中心
                    Transform::from_translation((custom_size / 2. * Vec2::new(1., -1.)).extend(0.)),
                ))
                // ! slot 只需一个 DragEnter 事件
                .observe(observe_slot)
                .id();
        }
    }

    // 放一个生成按钮
    // ! 未来可以使用 children![] 宏,简化嵌套, 现在却不行
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
    commands
        .spawn((
            Sprite::from_image(asset_server.load("items/paper.png")),
            Mesh2d(meshes.add(Circle::new(20.))),
            MeshMaterial2d(material.add(Color::srgb(1., 0., 0.))),
            Transform::from_xyz(300., 300., 0.),
        ))
        .observe(observe_circle::<Pointer<Move>>())
        .observe(observe_circle::<Pointer<Over>>());
}

/// 支持拖拽事件
fn observe_circle<E: Debug + Reflect + Clone>() -> impl Fn(Trigger<E>) {
    move |ev| {
        info!("observe_circle: {:?}", ev);
        let reflect = ev.event().try_as_reflect().unwrap();

        if let Some(trigger) = reflect.downcast_ref::<Pointer<DragEnd>>() {
            info!("DragEnd: {:?}", trigger.pointer_location);
        }
    }
}

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
