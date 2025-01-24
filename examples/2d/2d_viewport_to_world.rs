//! This example demonstrates how to use the `Camera::viewport_to_world_2d` method.

use bevy::{color::palettes::basic::WHITE, log::warn_once, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor.param_warn_once())
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform, &Transform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    // 如果 entity(camera) 没有父级, transform 就以 global_transform 为相对坐标
    // 所以我们可以看到 transform 很多默认值,并不是我们想像中的,如果不存在父级,坐标就是 GlobalTransform 的坐标
    // global_transform: GlobalTransform(Affine3A { matrix3: Mat3A { x_axis: Vec3A(1.0, 0.0, 0.0), y_axis: Vec3A(0.0, 1.0, 0.0), z_axis: Vec3A(0.0, 0.0, 1.0) }, translation: Vec3A(0.0, 0.0, 0.0) })
    // local_transform: Transform { translation: Vec3(0.0, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }
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
    // camera 是可以移动的, world 的可视泛围是相对于 camera 取景框的
    // 如果 camera 对准 Vec2:Zero 那么转换意义就不大了
    // 如果 camera 对准 Vec2(-100,-100),鼠标指向视窗(viewport) 中心点的时候,实际在世界坐标系中是偏移的
    // 所以通常,默认 camera 就是偏移的,每次都进行视窗转换,保证鼠标指向的是世界坐标系中的点
    // ---
    // camera.world_to_viewport 是世界坐标系到视窗坐标系的转换,这个方法只有 3D ,也只有 3D 有远近才需要转换成平面 UI

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
