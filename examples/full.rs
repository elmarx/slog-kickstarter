use crate::no_slog::log_via_log_crate;
use crate::sample_module::*;
use slog::{o, slog_info};
use slog_kickstarter::SlogKickstarter;
use slog_scope::set_global_logger;
use std::env;

fn main() {
    // initialize a root logger
    let root_logger = SlogKickstarter::new("logging-example")
        .with_debug_log_for("full::sample_module")
        .init();

    // set a global logger. The logger lives as long as the guard, so make sure the guard lives as long as the main-function
    let _guard = set_global_logger(root_logger.new(o!("scope" => "global")));

    let json_log_status = if env::var("RUST_LOG_JSON")
        .map(|v| v == "1")
        .unwrap_or_default()
    {
        "RUST_JSON_LOG=1 set, logging in JSON format"
    } else {
        "RUST_JSON_LOG=1 not set, logging in compact format"
    };

    // slog supports string formatting, and additional structured fields
    slog_info!(root_logger, "Hello World. {}", json_log_status; o!("type" => "example"));

    // example for a module with enforced debug-logging (set via `.with_debug_log_for()`)
    log_debug_mode(root_logger.new(o!("scope" => "module-specific logger")));
    log_via_log_crate();
    log_global();
}

mod sample_module {
    use slog::{o, slog_debug, slog_info, Logger};
    use slog_scope::logger;

    pub fn log_debug_mode(logger: Logger) {
        let example_value = 42;
        slog_debug!(logger, "This is debug log"; o!("example_value" => example_value));
    }

    pub fn log_global() {
        // Sometimes it's not feasible to pass logging-instances around, then you may use the global logger, which
        // has been made available via `slog-scope::set_global_logger`
        // Be aware: slog discourages this usage.
        slog_info!(logger(), "Without explicitly passed logger")
    }
}

/// An external module/crate that knows nothing about slog, and just uses the log-facade crate.
/// These case are covered by slog-stdlog
mod no_slog {
    use log::info;

    pub fn log_via_log_crate() {
        // logging via well-known `log` crate. If a library does not know about `slog`, or for your own legacy code.
        info!("Using the well-known 'log' crate")
    }
}
