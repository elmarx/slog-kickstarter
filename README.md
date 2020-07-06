[![Rust build](https://github.com/elmarx/slog-kickstarter/workflows/Rust/badge.svg)](https://github.com/elmarx/slog-kickstarter/actions?query=workflow%3ARust) [![crates.io badge](https://img.shields.io/crates/v/slog-kickstarter.svg)](https://crates.io/crates/slog-kickstarter) [![docs.rs badge](https://docs.rs/slog-kickstarter/badge.svg)](https://docs.rs/slog-kickstarter)

# Slog Kickstarter :rocket:

[Builder](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) to setup [slog](https://crates.io/crates/slog) easily.

Slog is really great, but to leverage it's features and flexibility, it requires more setup than importing a module and calling a function
(to be precise, it requires about 163 lines of code ;)).

Structured logging is great. You should not postpone introducing logging, but integrate it from the beginning, because 
retrofitting slog could require lots of changes. That's where Slog Kickstarter comes into play.

## Features

- switch between JSON-logging and [(compacted) terminal output](https://github.com/slog-rs/slog#terminal-output-example) via environment-variable (`RUST_LOG_JSON=1`)
- per-module debug-level (via `with_debug_log_for()`) — i.e. enable debugging for your own modules, but ignore debug-logs from 3rd-party crates
- logging configuration via `RUST_LOG=…`, as known from [env_logger](https://crates.io/crates/env_logger) (implemented by [slog-envlogger](https://crates.io/crates/slog-envlogger)), all preconfigured
- [log](https://crates.io/crates/log) support by default (unless explicitly disabled)

## Usage

See [basic.rs](examples/basic.rs) for a running example, and [full.rs](examples/full.rs) for all features. 

```rust
use slog::{o, slog_info};
use slog_kickstarter::SlogKickstarter;

fn main() {
    // initialize a root logger
    let root_logger = SlogKickstarter::new("logging-example").init();

    // slog supports string formatting, and additional structured fields
    slog_info!(root_logger, "Hello World!"; o!("type" => "example"));
}
```