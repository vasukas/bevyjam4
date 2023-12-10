use super::states::CloseMenu;
use super::states::MenuState;
use super::ui_const::UiConst;
use crate::utils::bevy_egui::*;
use crate::utils::math_algorithms::lerp;
use crate::utils::misc_utils::DurationDivF32 as _;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiSet;
use std::collections::VecDeque;
use std::time::Duration;

/// Send to show it to user
#[derive(Event, Clone)]
pub struct Message {
    pub header: String,
    pub text: String,
    pub ty: MessageType,
}

impl Message {
    pub fn notify(header: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            text: text.into(),
            ty: MessageType::Notification,
        }
    }

    pub fn delay(self, by: Duration) -> DelayedMessage {
        DelayedMessage { message: self, by }
    }
}

/// Look and behavior
#[derive(Clone, Copy)]
pub enum MessageType {
    /// Pop-up notification intended for gameplay
    Notification,

    /// Locks menu. Not real modal window - has only "OK" option.
    #[allow(unused)]
    ModalNotification,
}

impl MessageType {
    // TODO: this is all kinds of wrong, but works for now
    /// How long message should be shown.
    /// Doesn't include fade-in and fade-out durations.
    fn duration(&self, text: &str) -> Duration {
        let seconds_per_letter = 0.020;
        let seconds = seconds_per_letter * text.len() as f32;
        let seconds = seconds.clamp(3., 7.);
        Duration::from_secs_f32(seconds)
    }

    fn modal(&self) -> bool {
        match self {
            MessageType::ModalNotification => true,
            _ => false,
        }
    }
}

/// Sends message after specified amount of time passes
#[derive(Event)]
pub struct DelayedMessage {
    pub message: Message,
    pub by: Duration,
}

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Message>()
            .add_event::<DelayedMessage>()
            .init_resource::<MessageData>()
            .add_systems(
                PostUpdate,
                (
                    delayed_message,
                    draw_and_update_messages.before(EguiSet::ProcessOutput),
                )
                    .chain(),
            )
            .add_systems(OnExit(MenuState::ModalMessage), pop_modal_message);
    }
}

fn delayed_message(
    mut delayed: ResMut<Events<DelayedMessage>>,
    time: Res<Time<Real>>,
    mut data: Local<Vec<(Duration, Message)>>,
    mut messages: EventWriter<Message>,
) {
    data.extend(
        delayed
            .drain()
            .map(|delay| (time.elapsed() + delay.by, delay.message)),
    );

    data.retain(|(after, message)| {
        let retain = time.elapsed() < *after;
        if !retain {
            messages.send(message.clone());
        }
        retain
    });
}

fn fade_in_duration() -> Duration {
    Duration::from_secs_f32(0.500)
}

fn fade_out_duration() -> Duration {
    Duration::from_secs_f32(0.600)
}

#[derive(Resource, Default)]
struct MessageData {
    current_started_at: Option<Duration>,
    queue: VecDeque<Message>,
}

impl MessageData {
    fn pop(&mut self) {
        self.current_started_at = None;
        self.queue.pop_front();
    }
}

fn pop_modal_message(mut data: ResMut<MessageData>) {
    if let Some(message) = data.queue.front() {
        if message.ty.modal() {
            data.pop();
        }
    }
}

fn draw_and_update_messages(
    mut messages: ResMut<Events<Message>>,
    mut data: ResMut<MessageData>,
    time: Res<Time<Real>>,
    mut egui_ctx: EguiContexts,
    ui_const: UiConst,
    mut close_menu: EventWriter<CloseMenu>,
    mut menu_state: ResMut<NextState<MenuState>>,
    primary_window: Query<(), With<PrimaryWindow>>,
) {
    let ui_const = ui_const.scale();
    let margin = 20. * ui_const;
    let text_size = 28. * ui_const;
    let popup_offset = Vec2::new(0., -20.) * ui_const;

    //

    if primary_window.is_empty() {
        // Don't panic on exiting app.
        // This happens only when ctx_mut() is used in PostUpdate after window was closed.
        return;
    }

    let data = &mut *data;

    data.queue.extend(messages.drain());

    //

    if let Some(message) = data.queue.front() {
        if data.current_started_at.is_none() && message.ty.modal() {
            menu_state.set(MenuState::ModalMessage);
        }

        let started_at = *data.current_started_at.get_or_insert(time.elapsed());
        let passed = time.elapsed().saturating_sub(started_at);

        let text_duration = message.ty.duration(&message.text);
        let total_duration = text_duration + fade_in_duration() + fade_out_duration();

        if passed >= total_duration && !message.ty.modal() {
            data.pop();
        } else {
            let bg_alpha = 0.7;
            let bg_color_start = Color::rgb(0.3, 0.6, 0.6);
            let bg_color_main = Color::BLACK;

            let text_color = match message.ty {
                MessageType::Notification => Color::WHITE,
                MessageType::ModalNotification => Color::WHITE,
            };

            let fade_in;
            let alpha;

            if passed < fade_in_duration() {
                fade_in = passed.div_dur_f32(fade_in_duration());
                alpha = 1.;
            } else if passed < fade_in_duration() + text_duration {
                fade_in = 1.;
                alpha = 1.;
            } else {
                fade_in = 1.;
                alpha = 1.
                    - passed
                        .saturating_sub(fade_in_duration() + text_duration)
                        .div_dur_f32(fade_out_duration());
            };

            if message.ty.modal() {
                EguiPopup {
                    name: "draw_messages",
                    anchor: egui::Align2::CENTER_CENTER,
                    order: egui::Order::Foreground,
                    ..default()
                }
                .show(egui_ctx.ctx_mut(), |ui| {
                    let style = ui.style_mut();
                    style.spacing.window_margin = egui::Margin::same(margin);
                    style.visuals.window_fill = lerp(bg_color_start, bg_color_main, fade_in)
                        .with_a(alpha * bg_alpha)
                        .to_egui();
                    style.visuals.window_stroke = egui::Stroke::NONE; // no border

                    egui::Frame::popup(style).show(ui, |ui| {
                        if message.header.is_empty() {
                            ui.label(egui::RichText::new(&message.text).strong().size(text_size));
                        } else {
                            ui.label(egui::RichText::new(&message.header).heading().strong());
                            ui.label(&message.text);
                        }

                        if ui.button("OK").clicked() {
                            close_menu.send_default();
                        }
                    });
                });
            } else {
                EguiPopup {
                    name: "draw_messages",
                    anchor: egui::Align2::CENTER_BOTTOM,
                    offset: popup_offset,
                    order: egui::Order::Foreground,
                    interactable: false,
                    background: false,
                    ..default()
                }
                .show(egui_ctx.ctx_mut(), |ui| {
                    let style = ui.style_mut();
                    style.spacing.window_margin = egui::Margin::same(margin);
                    style.visuals.window_fill = lerp(bg_color_start, bg_color_main, fade_in)
                        .with_a(alpha * bg_alpha)
                        .to_egui();
                    style.visuals.window_stroke = egui::Stroke::NONE; // no border

                    egui::Frame::popup(style).show(ui, |ui| {
                        ui.visuals_mut().override_text_color =
                            text_color.with_a(alpha).to_egui().into();

                        if message.header.is_empty() {
                            ui.label(egui::RichText::new(&message.text).strong().size(text_size));
                        } else {
                            ui.label(egui::RichText::new(&message.header).heading().strong());
                            ui.label(&message.text);
                        }
                    });
                });
            }
        }
    }
}
