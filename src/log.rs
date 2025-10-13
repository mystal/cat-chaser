use bevy::log::{Level, LogPlugin};

pub fn log_plugin() -> LogPlugin {
    // Get the crate name to configure logging.
    let (crate_name, _) = module_path!()
        .split_once("::")
        .unwrap_or((module_path!(), ""));

    // Configure logging.
    let mut plugin = LogPlugin::default();
    if cfg!(feature = "verbose_logs") {
        plugin.filter.push_str(&format!(",info,{crate_name}=trace"));
        plugin.level = Level::TRACE;
    } else if cfg!(debug_assertions) {
        plugin.filter.push_str(&format!(",info,{crate_name}=debug"));
        plugin.level = Level::DEBUG;
    }
    plugin
}
