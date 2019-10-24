/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *                     Hikaru Terazono <3c1u@vulpesgames.tokyo>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

use crate::Logger;
use colored::*;

/// Create a custom logger.
///
/// * `tag` - String to use as prefix (after scope).
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - Use `eprintln` instead `eprint` (default: true).
pub fn custom(tag: &ColoredString, message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::custom(std::io::stderr(), tag, message, scope, ln).unwrap();
}

/// Simple header/title for CLIs.
///
/// * `name` - Name of the project.
/// * `version` - Include also the version.
pub fn head(name: &str, icon: Option<&str>, version: Option<&str>) {
    Logger::head(std::io::stderr(), name, icon, version).unwrap();
}

/// Informational message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn info(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::info(std::io::stderr(), message, scope, ln).unwrap();
}

/// Succesfull operation.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn success(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::success(std::io::stderr(), message, scope, ln).unwrap();
}

/// Warn message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn warn(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::warn(std::io::stderr(), message, scope, ln).unwrap();
}

/// Error message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn error(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::error(std::io::stderr(), message, scope, ln).unwrap();
}

/// Waiting for something.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn wait(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::wait(std::io::stderr(), message, scope, ln).unwrap();
}

/// Something finished.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn done(message: &str, scope: Option<&str>, ln: Option<bool>) {
    Logger::done(std::io::stderr(), message, scope, ln).unwrap();
}

/// Put the cursor at the init of the actual line.
pub fn remove() {
    Logger::remove(std::io::stderr()).unwrap();
}
