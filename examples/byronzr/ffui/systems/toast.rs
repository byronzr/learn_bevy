use crate::define::FontHandle;
use crate::define::ProcessState;
use crate::define::ToastDisappear;
use crate::define::ToastMaker;
use bevy::prelude::*;

// 使用 #[macro_export] 使宏可以被外部模块导入

pub fn toast_consumer(
    mut commands: Commands,
    mut toast_message: ResMut<ProcessState>,
    font: Res<FontHandle>,
) {
    if !toast_message.toast_message.is_empty() {
        for message in toast_message.toast_message.drain(..) {
            //info!("{}", message);
            commands.spawn((
                ToastMaker(Timer::from_seconds(3.0, TimerMode::Once)),
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(40.0),
                    bottom: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderRadius::all(Val::Px(5.0)),
                BackgroundColor(Color::srgb_u8(150, 150, 0).with_alpha(0.2)),
                children![(
                    Text::new(message),
                    TextFont {
                        font: font.0.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ));
        }
    }
}

pub fn toast_animate(
    mut commands: Commands,
    time: Res<Time>,
    mut toast_query: Query<(
        Entity,
        &mut ToastMaker,
        &mut Node,
        &mut BackgroundColor,
        &Children,
        Option<&mut ToastDisappear>,
    )>,
    mut text_query: Query<&mut TextColor>,
) {
    for (entity, mut toast_maker, mut node, mut bgcolor, children, disappear) in
        toast_query.iter_mut()
    {
        // if the toast has a ToastDisappear component, it means it should be removed after a certain time
        if let Some(mut timer) = disappear {
            if timer.0.tick(time.delta()).just_finished() {
                // despawn the toast after the timer finishes
                commands.entity(entity).despawn();
                continue;
            } else {
                // animate the toast to fade out
                let rate = timer.0.elapsed_secs() / timer.0.duration().as_secs_f32();

                // 使用 ease-out 曲线 (更自然的淡出效果)
                // 公式: 1 - (1 - t)^2
                let ease_out_rate = 1.0 - (1.0 - rate).powi(2);

                // 其他可选的缓动函数:
                // ease-in: t^2
                // let ease_in_rate = rate * rate;

                // ease-in-out: 先慢后快再慢
                // let ease_in_out_rate = if rate < 0.5 {
                //     2.0 * rate * rate
                // } else {
                //     1.0 - 2.0 * (1.0 - rate).powi(2)
                // };

                // 对于淡出效果，通常使用 ease-out 更自然
                let rate = ease_out_rate;
                let alpha = 1.0 - ease_out_rate;

                // 应用透明度动画到背景
                node.bottom = Val::Px(40.0 + 40.0 * rate);
                bgcolor.0.set_alpha(alpha * 0.2);

                // 应用透明度动画到文字
                for child in children.iter() {
                    if let Ok(mut text_color) = text_query.get_mut(child) {
                        let mut color = text_color.0;
                        // 文字透明度相对于原始透明度按比例变化
                        color.set_alpha(alpha);
                        text_color.0 = color;
                    }
                }
            }
        }

        if toast_maker.0.tick(time.delta()).just_finished() {
            // despawn the toast after the timer finishes
            commands
                .entity(entity)
                .insert(ToastDisappear(Timer::from_seconds(1.0, TimerMode::Once)));
        }
    }
}

pub fn toast_receiver(mut process_state: ResMut<ProcessState>) {
    // 从通道中接收消息
    let Ok(message) = process_state.toast_rx.try_recv() else {
        return;
    };
    // 将消息添加到 toast_message 中
    process_state.toast_message.push(message);
}
