use bevy::log::Level;
use bevy::prelude::*;
use bevy::utils::tracing::field;
use bevy::utils::tracing::Subscriber;
use std::fmt::Write as _;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

#[derive(Event, Clone)]
pub struct LogMessage {
    pub level: Level,
    pub text: String,
}

/// Butchered version of [`bevy::log::LogPlugin`], which can output messages as bevy events
pub struct LogPlugin {
    /// Filters logs using the [`EnvFilter`] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self {
            filter: "wgpu=error,naga=warn".to_string(),
            level: Level::INFO,
        }
    }
}

impl LogPlugin {
    pub fn with_filter(mut self, crate_name: &str, level: Level) -> Self {
        let level_name = match level {
            Level::TRACE => "trace",
            Level::DEBUG => "debug",
            Level::INFO => "info",
            Level::WARN => "warn",
            Level::ERROR => "error",
        };
        self.filter += &format!(",{crate_name}={level_name}");
        self
    }
}

const EVENT_CHANNEL_CAPACITY: usize = 1024;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        let default_filter = { format!("{},{}", self.level, self.filter) };
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap();
        let subscriber = Registry::default().with(filter_layer);

        let fmt_layer = tracing_subscriber::fmt::Layer::default()
            .map_event_format(|f| f.with_line_number(true));
        #[cfg(target_arch = "wasm32")]
        let fmt_layer = fmt_layer.map_event_format(|f| f.without_time());
        #[cfg(not(target_arch = "wasm32"))]
        let fmt_layer = fmt_layer.with_writer(std::io::stderr);
        let subscriber = subscriber.with(fmt_layer);

        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();
        #[cfg(target_arch = "wasm32")]
        let subscriber = subscriber.with(tracing_wasm::WASMLayer::new(
            tracing_wasm::WASMLayerConfig::default(),
        ));

        let (sender, receiver) = crossbeam_channel::bounded(EVENT_CHANNEL_CAPACITY);
        app.add_event::<LogMessage>()
            .insert_resource(LogReceiver(receiver))
            .add_systems(Update, transceive_events);

        let events_layer = SendLayer { sender };
        let subscriber = subscriber.with(events_layer);

        LogTracer::init().expect("Nothing else can set another logger");
        bevy::utils::tracing::subscriber::set_global_default(subscriber)
            .expect("Nothing else can set another logger");
    }
}

#[derive(Resource)]
struct LogReceiver(crossbeam_channel::Receiver<LogMessage>);

fn transceive_events(receiver: Res<LogReceiver>, mut events: EventWriter<LogMessage>) {
    for event in receiver.0.try_iter() {
        events.send(event);
    }
}

struct SendLayer {
    sender: crossbeam_channel::Sender<LogMessage>,
}

impl<S: Subscriber> Layer<S> for SendLayer {
    fn on_event(
        &self,
        event: &bevy::utils::tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut text = format!("{}: ", event.metadata().module_path().unwrap_or("???"));

        event.record(&mut FieldVisitor(&mut text));

        let _ = self.sender.send(LogMessage {
            level: *event.metadata().level(),
            text,
        });
    }
}

struct FieldVisitor<'a>(&'a mut String);

impl field::Visit for FieldVisitor<'_> {
    fn record_debug(&mut self, field: &field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(self.0, "{value:?}").unwrap();
        } else {
            write!(self.0, "{{{}={:?}}}", field.name(), value).unwrap();
        }
    }
}
