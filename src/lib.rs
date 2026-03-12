//! Loco.rs 自定义日志初始化器
//!
//! 使用 `tracing-appender-localtime` 替代标准 `tracing-appender`，
//! 实现日志文件名使用本地时间而非 UTC 时间。
//!
//! # 用法
//!
//! 在 `src/app.rs` 的 `Hooks::init_logger` 中调用：
//!
//! ```rust,ignore
//! fn init_logger(ctx: &AppContext) -> Result<bool> {
//!     let _ = loco_logger_localtime::init::<App>(&ctx.config.logger);
//!     Ok(true)
//! }
//! ```
//!
//! 然后可以从 `src/lib.rs` 中移除 `pub mod logger;` 以及对应的 `src/logger.rs` 文件。

use std::sync::OnceLock;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt, fmt::MakeWriter, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
    Registry,
};

use loco_rs::{
    app::Hooks,
    config,
    logger::{Format, LogLevel, Rotation},
    Error, Result,
};

const MODULE_WHITELIST: &[&str] = &[
    "loco_rs",
    "sea_orm_migration",
    "tower_http",
    "sqlx::query",
    "playground",
    "loco_gen",
];

static NONBLOCKING_WORK_GUARD_KEEP: OnceLock<WorkerGuard> = OnceLock::new();

/// 初始化日志系统。
///
/// 日志过滤规则：
/// 1. 如果设置了 `RUST_LOG` 环境变量，使用该过滤器
/// 2. 如果配置了 `override_filter`，使用该过滤器（忽略其他）
/// 3. 否则使用 `MODULE_WHITELIST` + 应用名称，按 `config.level` 过滤
///
/// # Errors
///
/// 无法初始化日志或设置 appender 时返回错误。
pub fn init<H: Hooks>(config: &config::Logger) -> Result<()> {
    let mut layers: Vec<Box<dyn Layer<Registry> + Sync + Send>> = Vec::new();

    if let Some(file_appender_config) = config.file_appender.as_ref() {
        if file_appender_config.enable {
            let dir = file_appender_config
                .dir
                .as_ref()
                .map_or_else(|| "./logs".to_string(), ToString::to_string);

            let mut rolling_builder = tracing_appender::rolling::Builder::default()
                .max_log_files(file_appender_config.max_log_files);

            rolling_builder = match file_appender_config.rotation {
                Rotation::Minutely => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::MINUTELY)
                }
                Rotation::Hourly => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::HOURLY)
                }
                Rotation::Daily => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::DAILY)
                }
                Rotation::Never => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::NEVER)
                }
            };

            let file_appender = rolling_builder
                .filename_prefix(
                    file_appender_config
                        .filename_prefix
                        .as_ref()
                        .map_or_else(String::new, ToString::to_string),
                )
                .filename_suffix(
                    file_appender_config
                        .filename_suffix
                        .as_ref()
                        .map_or_else(String::new, ToString::to_string),
                )
                .build(dir)
                .map_err(Error::msg)?;

            let file_appender_layer = if file_appender_config.non_blocking {
                let (non_blocking_file_appender, work_guard) =
                    tracing_appender::non_blocking(file_appender);
                NONBLOCKING_WORK_GUARD_KEEP
                    .set(work_guard)
                    .map_err(|_| Error::string("cannot lock for appender"))?;
                init_layer(
                    non_blocking_file_appender,
                    &file_appender_config.format,
                    false,
                )
            } else {
                init_layer(file_appender, &file_appender_config.format, false)
            };
            layers.push(file_appender_layer);
        }
    }

    if config.enable {
        let stdout_layer = init_layer(std::io::stdout, &config.format, true);
        layers.push(stdout_layer);
    }

    if !layers.is_empty() {
        let env_filter = init_env_filter::<H>(config.override_filter.as_ref(), &config.level);
        tracing_subscriber::registry()
            .with(layers)
            .with(env_filter)
            .init();
    }
    Ok(())
}

fn init_env_filter<H: Hooks>(override_filter: Option<&String>, level: &LogLevel) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .or_else(|_| {
            override_filter.map_or_else(
                || {
                    EnvFilter::try_new(
                        MODULE_WHITELIST
                            .iter()
                            .map(|m| format!("{m}={level}"))
                            .chain(std::iter::once(format!("{}={}", H::app_name(), level)))
                            .collect::<Vec<_>>()
                            .join(","),
                    )
                },
                EnvFilter::try_new,
            )
        })
        .expect("logger initialization failed")
}

fn init_layer<W2>(
    make_writer: W2,
    format: &Format,
    ansi: bool,
) -> Box<dyn Layer<Registry> + Sync + Send>
where
    W2: for<'writer> MakeWriter<'writer> + Sync + Send + 'static,
{
    match format {
        Format::Compact => fmt::Layer::default()
            .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
            .with_ansi(ansi)
            .with_writer(make_writer)
            .compact()
            .boxed(),
        Format::Pretty => fmt::Layer::default()
            .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
            .with_ansi(ansi)
            .with_writer(make_writer)
            .pretty()
            .boxed(),
        Format::Json => fmt::Layer::default()
            .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
            .with_ansi(ansi)
            .with_writer(make_writer)
            .json()
            .boxed(),
    }
}
