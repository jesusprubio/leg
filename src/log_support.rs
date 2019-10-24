/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *                     Hikaru Terazono <3c1u@vulpesgames.tokyo>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

//! log crate support using env_logger crate

use crate::Logger;
use env_logger::Builder;

/// Initialises the global logger. Panics when fails.
#[inline]
pub fn init() {
    try_init().unwrap();
}

/// Initialises the global logger with a given environmental variable name.
/// Panics when fails.
#[inline]
pub fn init_custom_env(env_name: &str) {
    try_init_custom_env(env_name).unwrap();
}

/// Try to initialise the global logger.
pub fn try_init() -> Result<(), log::SetLoggerError> {
    try_init_custom_env("RUST_LOG")
}

/// Try to initialise the global logger with a given environmental variable name.
pub fn try_init_custom_env(env_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_builder();

    if let Ok(s) = std::env::var(env_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Returns a `env_logger::Builder` with a formatter.
pub fn formatted_builder() -> Builder {
    let mut builder = Builder::new();

    builder.format(|writer, record| {
        use log::Level;

        (match record.level() {
            Level::Error => Logger::error,
            Level::Warn => Logger::warn,
            Level::Trace => Logger::trace,
            Level::Debug => Logger::debug,
            _ => Logger::info,
        })(
            writer,
            &format!("{}", record.args()),
            Some(record.target()),
            None,
        )?;

        Ok(())
    });

    builder
}
