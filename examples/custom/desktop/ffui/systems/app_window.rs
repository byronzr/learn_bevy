use bevy::prelude::*;
pub fn move_or_resize_windows(
    mut windows: Query<&mut Window>,
    input: Res<ButtonInput<MouseButton>>,
) {
    // Both `start_drag_move()` and `start_drag_resize()` must be called after a
    // left mouse button press as done here.
    //
    // winit 0.30.5 may panic when initiated without a left mouse button press.
    if input.just_pressed(MouseButton::Left) {
        for mut window in windows.iter_mut() {
            window.start_drag_move();
        }
    }
}
