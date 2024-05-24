use bevy::log::{Level, LogPlugin};

pub fn log_plugin() -> LogPlugin {
    // Configure logging.
    let mut plugin = LogPlugin::default();
    // TODO: Is there a way to get the crate name automatically?
    if cfg!(feature = "verbose_logs") {
        plugin.filter.push_str(",info,cat_chaser=trace");
        plugin.level = Level::TRACE;
    } else if cfg!(debug_assertions) {
        plugin.filter.push_str(",info,cat_chaser=debug");
        plugin.level = Level::DEBUG;
    }
    plugin
}
