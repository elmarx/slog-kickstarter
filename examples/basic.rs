use slog::{o, slog_info};
use slog_kickstarter::SlogKickstarter;

fn main() {
    // initialize a root logger
    let root_logger = SlogKickstarter::new("logging-example").init();

    // slog supports string formatting, and additional structured fields
    slog_info!(root_logger, "Hello World!"; o!("type" => "example"));
}
