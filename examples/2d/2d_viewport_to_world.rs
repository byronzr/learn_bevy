//! This example demonstrates how to use the `Camera::viewport_to_world_2d` method.

use bevy::{color::palettes::basic::WHITE, log::warn_once, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor.param_warn_once())
        .run();
}

// Transform 相对坐标系(Entity嵌套)
// GlobalTransform 绝对坐标系(世界坐标系),脱离父子关系
fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform, &Transform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform, local_transform) = *camera_query;
    warn_once!("global_transform: {camera_transform:?}");
    warn_once!("local_transform: {local_transform:?}");

    // 获得 App 的 window
    let Ok(window) = windows.get_single() else {
        println!("window not found");
        return;
    };

    // 获得鼠标的位置(不包括 title bar mac中),所以鼠标位置是基于 Canvas 的
    let Some(cursor_position) = window.cursor_position() else {
        //warn!("cursor_position not found");
        warn_once!("cursor_position not found");
        return;
    };

    // Calculate a world position based on the cursor's position.
    // 将视窗坐标系转换成世界坐标系
    // 如果有 viewport_to_world_2d 方法,当然就会有 world_to_viewport 方法
    // 注意: 没有 world_to_viewport_2d 方法,因为 2d 是不需要转换的,因为没有深度(z)
    // 在 2d 场景中,如果鼠标(或其它)超出屏幕,则不需要转换,而未超出屏幕的部分,其本身就是世界坐标系
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        warn!("viewport_to_world_2d failed");
        return;
    };

    // 画个小圆圈跟随鼠标
    gizmos.circle_2d(point, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
