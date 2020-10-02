//! Slog Kickstarter. Easily sets up slog for structured logging.
//!
//! - enables JSON logging if `RUST_LOG_JSON=1` (i.e. set `RUST_LOG_JSON=1` for your deployment, or put `ENV RUST_LOG_JSON=1` into your Dockerfile)
//! - inits and configures stdlogger, so crates using `info!()` from the (default) [log-crate](https://crates.io/crates/log) can log messages
//! - allows to enable debugging for given modules (typically your own modules)
//! - sets default loglevel 'Info'
//! - supports well-known `[RUST_LOG](https://crates.io/crates/env_logger)`
//!
//! Usage:
//! ```rust
//! use slog_kickstarter::SlogKickstarter;
//! use slog_scope::set_global_logger;
//! use slog::o;
//!
//! let root_logger = SlogKickstarter::new("service-name").init();
//! let _guard = set_global_logger(root_logger.new(o!("scope" => "global")));
//! ```

use chrono::prelude::*;
use chrono::Local;
use slog::Record;
use slog::{o, Drain, FilterLevel, Fuse, Logger};
use slog::{FnValue, PushFnValue};
use slog_async::Async;
use slog_envlogger::LogBuilder as EnvLogBuilder;
use slog_json::Json;
use slog_term::{CompactFormat, TermDecorator};
use std::env;

/// the actual slog builder
pub struct SlogKickstarter {
    default_filter_level: FilterLevel,
    debug_modules: Vec<&'static str>,
    service_name: String,
    init_std_log: bool,
    use_json_logging: bool,
}

impl SlogKickstarter {
    #[must_use]
    /// initialize the log-builder with a name for your service
    pub fn new<S: Into<String>>(service_name: S) -> Self {
        let use_json_logging = env::var("RUST_LOG_JSON")
            .map(|v| v == "1")
            .unwrap_or_default();

        Self {
            default_filter_level: FilterLevel::Info,
            debug_modules: vec![],
            service_name: service_name.into(),
            init_std_log: true,
            use_json_logging,
        }
    }

    /// enable debug-log for the given module. May be called multiple times to add debug logging
    /// for multiple modules
    pub fn with_debug_log_for(&mut self, module_name: &'static str) -> &mut Self {
        self.debug_modules.push(module_name);
        self
    }

    /// set a default loglevel
    pub fn with_default_level(&mut self, level: FilterLevel) -> &mut Self {
        self.default_filter_level = level;
        self
    }

    /// enforce JSON logging
    ///
    /// this should typically be set via `RUST_LOG_JSON=1`
    pub fn with_json_logging(&mut self) -> &mut Self {
        self.use_json_logging = true;
        self
    }

    /// enforce **no** JSON logging
    ///
    /// this should typically be set via `RUST_LOG_JSON=0`, or just leaving out `RUST_LOG_JSON`,
    /// as this is the default
    pub fn without_json_logging(&mut self) -> &mut Self {
        self.use_json_logging = false;
        self
    }

    /// do not initialize stdlog, i.e. disable slog for logs from [log](https://crates.io/crates/log)
    pub fn without_stdlog(&mut self) -> &mut Self {
        self.init_std_log = false;
        self
    }

    /// initialize the logger based on the builder
    #[must_use]
    pub fn init(&self) -> Logger {
        // output in json-format iff RUST_LOG_JSON=1
        let drain = if self.use_json_logging {
            self.setup_json_logging()
        } else {
            self.setup_term_logging()
        };

        if self.init_std_log {
            let _guard = slog_stdlog::init();
        }

        slog::Logger::root(
            drain,
            o!("version" => env!("CARGO_PKG_VERSION"), "service" => self.service_name.to_owned(), "log_type" => "application", "application_type" => "service", "module" => FnValue(move |info| {
                info.module().to_string()
            })
            ),
        )
    }

    fn setup_json_logging(&self) -> Fuse<Async> {
        let drain = Json::new(std::io::stdout())
            .add_key_value(o!(
            "@timestamp" => PushFnValue(move |_ : &Record, ser| {
                ser.emit(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true))
            }),
            "loglevel" => FnValue(move |rinfo : &Record| {
                rinfo.level().as_str()
            }),
            "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
            ))
            .build()
            .fuse();

        let builder = EnvLogBuilder::new(drain)
            // set default log-level 'info'…
            .filter(None, self.default_filter_level);

        let builder = self.debug_modules.iter().fold(builder, |b, &module_name| {
            b.filter(Some(module_name), FilterLevel::Debug)
        });

        let drain = builder
            //but override with RUST_LOG (if given)
            .parse(env::var("RUST_LOG").unwrap_or_default().as_str())
            .build()
            .fuse();

        slog_async::Async::new(drain).build().fuse()
    }

    fn setup_term_logging(&self) -> Fuse<Async> {
        let decorator = TermDecorator::new().build();
        let drain = CompactFormat::new(decorator).build().fuse();

        // builder with given default loglevel as default for all modules
        let builder = EnvLogBuilder::new(drain).filter(None, self.default_filter_level);

        // set debug-exceptions for specific modules
        let builder = self.debug_modules.iter().fold(builder, |b, &module_name| {
            b.filter(Some(module_name), FilterLevel::Debug)
        });

        let drain = builder
            // override with RUST_LOG (if given)
            .parse(env::var("RUST_LOG").unwrap_or_default().as_str())
            .build()
            .fuse();

        slog_async::Async::new(drain).build().fuse()
    }
}
