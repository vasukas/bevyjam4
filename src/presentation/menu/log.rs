use crate::{
    app::settings::AppSettings,
    utils::{
        for_crate::bevy_egui::BevyEguiColor as _, math_algorithms::map_linear_range,
        misc_utils::invert_color, plugins::log_plugin::LogMessage,
    },
};
use bevy::{log::Level, prelude::*};
use bevy_egui::{egui, EguiContexts};
use std::time::Duration;

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_messages);
    }
}

#[derive(Default)]
struct DebugLog {
    // circular buffer: message, time when added
    messages: Vec<(Option<LogMessage>, Duration)>,
    // index into messages where to add next message
    next: usize,

    // has any non-expired messages
    has_any: bool,
}

fn log_messages(
    mut egui_ctx: EguiContexts,
    time: Res<Time<Real>>,
    mut messages: ResMut<Events<LogMessage>>,
    mut data: Local<DebugLog>,
    settings: Res<AppSettings>,
) {
    const DURATION: f32 = 5.; // how long message is shown
    const FADE_START: f32 = 4.; // when message starts fade to transparency
    const FONT_SIZE_INCREASE: f32 = 5.; // no idea how to read real text size, so just use this
    let background_color = Color::BLACK.with_a(0.7);

    const IGNORED_MESSAGES: &'static [&'static str] = &[];

    let level_color = |level: Level| match level {
        Level::TRACE => Color::WHITE,
        Level::DEBUG => Color::CYAN,
        Level::INFO => Color::GREEN,
        Level::WARN => Color::ORANGE_RED,
        Level::ERROR => Color::RED,
    };

    egui::Area::new("log_messages")
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
        .order(egui::Order::Tooltip)
        .interactable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            // update window size
            {
                // TODO: size is probably not entirely correct (esp. considering FONT_SIZE_INCREASE),
                // plus message can be (and should be!) wrapped, taking two or more rows,
                // while this expects strictly row per message

                let style = ui.style();
                let font_size = style.text_styles.get(&egui::TextStyle::Body).unwrap().size;
                let font_size = font_size + FONT_SIZE_INCREASE;

                let window_height = ui.available_size().y;
                let max_messages = (window_height / font_size) as usize;

                if data.messages.len() != max_messages {
                    data.messages.clear();
                    data.messages.resize(max_messages, default());

                    data.next = 0;
                }
            }

            // add new messages
            {
                let data = &mut *data;
                'evloop: for event in messages.drain() {
                    if !settings.log.show_all {
                        if !match event.level {
                            Level::TRACE | Level::DEBUG | Level::INFO => false,
                            Level::WARN | Level::ERROR => settings.log.show_errors,
                        } {
                            continue;
                        }
                    }

                    for ignored in IGNORED_MESSAGES {
                        if event.text.starts_with(ignored) {
                            continue 'evloop;
                        }
                    }

                    for line in event.text.split('\n') {
                        let event = LogMessage {
                            level: event.level,
                            text: format!("LOG: {line}"),
                        };

                        data.messages[data.next] = (Some(event), time.elapsed());
                        data.next = (data.next + 1) % data.messages.len();
                    }
                }
                data.has_any = true;
            }

            // remove old messages & update state
            {
                let data = &mut *data;
                data.has_any = false;
                for (message, received_at) in data.messages.iter_mut() {
                    if message.is_some() {
                        let passed = time.elapsed().saturating_sub(*received_at);
                        if passed.as_secs_f32() < DURATION {
                            data.has_any = true;
                        } else {
                            *message = None;
                        }
                    }
                }
                if !data.has_any {
                    data.next = 0;
                }
            }

            // draw messages
            {
                let newest_message = data
                    .next
                    .checked_sub(1)
                    .unwrap_or(data.messages.len().saturating_sub(1));

                let last_index = data
                    .messages
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(i, message)| message.0.as_ref().map(|_| i))
                    .unwrap_or_default();

                for (i, (message, received_at)) in data.messages.iter().enumerate() {
                    if let Some(message) = message {
                        let t =
                            time.elapsed().saturating_sub(*received_at).as_secs_f32() / DURATION;
                        let alpha = if t < FADE_START {
                            1.
                        } else {
                            map_linear_range(t, FADE_START, DURATION, 1., 0., true)
                        };

                        let color = level_color(message.level).with_a(alpha);

                        let background_color = if i == newest_message {
                            invert_color(color).with_a(background_color.a())
                        } else {
                            background_color
                        };

                        ui.label(
                            egui::RichText::new(&message.text)
                                .small()
                                .color(color.to_egui())
                                .background_color(background_color.to_egui()),
                        );
                    } else {
                        if i < last_index {
                            ui.small(" ");
                        } else {
                            break;
                        }
                    }
                }
            }
        });
}
