/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *                     Hikaru Terazono <3c1u@vulpesgames.tokyo>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

use colored::*;

mod printer;
pub use printer::*;

#[cfg(feature = "log_crate")]
mod log_support;
#[cfg(feature = "log_crate")]
pub use log_support::*;

/// Logger.
pub struct Logger;

impl Logger {
    /// Create a custom logger.
    ///
    /// * `writer` - A writer to output a log.
    /// * `tag` - String to use as prefix (after scope).
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - Use `eprintln` instead `eprint` (default: true).
    pub fn custom<W: std::io::Write>(
        writer: W,
        tag: &ColoredString,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        let pref = match scope {
            None => "".to_string(),
            Some(p) => format!("[{}]", p),
        };

        let mut writer = writer;

        write!(writer, "{} {} {}", pref.dimmed(), tag, message)?;

        if ln.unwrap_or(true) {
            writeln!(writer)?;
        }

        Ok(())
    }

    /// Simple header/title for CLIs.
    ///
    /// * `writer` - A writer to output a log.
    /// * `name` - Name of the project.
    /// * `version` - Include also the version.
    pub fn head<W: std::io::Write>(
        writer: W,
        name: &str,
        icon: Option<&str>,
        version: Option<&str>,
    ) -> std::io::Result<()> {
        let ver = match version {
            None => "".to_string(),
            Some(v) => format!("\n\t(v{})\n", v),
        };

        let ico = match icon {
            None => "",
            Some(i) => i,
        };

        let mut writer = writer;

        write!(
            writer,
            "\n\t{} {}{}",
            ico,
            name.bold().underline(),
            ver.dimmed()
        )
    }

    /// Informational message.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn info<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"ℹ".blue().bold(), message, scope, ln)
    }

    /// Succesfull operation.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn success<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"✔".green().bold(), message, scope, ln)
    }

    /// Warn message.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn warn<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"⚠".yellow().bold(), message, scope, ln)
    }

    /// Error message.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn error<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"✖".red().bold(), message, scope, ln)
    }

    /// Waiting for something.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn wait<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"…".magenta().bold(), message, scope, ln)
    }

    /// Something finished.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn done<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"☒".cyan().bold(), message, scope, ln)
    }

    /// Trace log.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn trace<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"🔍".blue().bold(), message, scope, ln)
    }

    /// Debug log.
    ///
    /// * `writer` - A writer to output a log.
    /// * `message` - String to print.
    /// * `scope` - Preffix to append.
    /// * `ln` - To print in the same line instead  (default: false).
    pub fn debug<W: std::io::Write>(
        writer: W,
        message: &str,
        scope: Option<&str>,
        ln: Option<bool>,
    ) -> std::io::Result<()> {
        Self::custom(writer, &"🐛".green().bold(), message, scope, ln)
    }

    /// Put the cursor at the init of the actual line.
    ///
    /// * `writer` - A writer to output a log.
    pub fn remove<W: std::io::Write>(writer: W) -> std::io::Result<()> {
        let mut writer = writer;

        write!(writer, "\r")
    }
}
