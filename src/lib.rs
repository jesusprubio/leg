/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */
use async_std::{eprint, eprintln, println};

use colored::*;

/// Create a custom logger.
///
/// * `tag` - String to use as prefix (after scope).
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - Use `eprintln` instead `eprint` (default: true).
pub async fn custom(tag: &ColoredString, message: &str, scope: Option<&str>, ln: Option<bool>) {
    let pref = match scope {
        None => "".to_string(),
        Some(p) => format!("[{}]", p),
    };

    let to_print = format!("{} {} {}", pref.dimmed(), tag, message);

    match ln {
        Some(false) => eprint!("{}", to_print).await,
        _ => eprintln!("{}", to_print).await,
    }
}

/// Simple header/title for CLIs.
///
/// * `name` - Name of the project.
/// * `version` - Include also the version.
pub async fn head(name: &str, icon: Option<&str>, version: Option<&str>) {
    let ver = match version {
        None => "".to_string(),
        Some(v) => format!("\n\t(v{})\n", v),
    };

    let ico = match icon {
        None => "",
        Some(i) => i,
    };

    eprintln!("\n\t{} {}{} ", ico, name.bold().underline(), ver.dimmed()).await;
}

/// Informational message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn info(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"ℹ".blue().bold(), message, scope, ln).await;
}

/// Succesfull operation.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn success(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"✔".green().bold(), message, scope, ln).await;
}

/// Warn message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn warn(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"⚠".yellow().bold(), message, scope, ln).await;
}

/// Error message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn error(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"✖".red().bold(), message, scope, ln).await;
}

/// Waiting for something.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn wait(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"…".magenta().bold(), message, scope, ln).await;
}

/// Something finished.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub async fn done(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"☒".cyan().bold(), message, scope, ln).await;
}

/// Put the cursor at the init of the actual line.
pub async fn remove() {
    eprint!("\r").await;
}

/// Print the final result to the standard output.
///
/// * `message` - String to print.
pub async fn result(message: &str) {
    println!("{}", message).await;
}
