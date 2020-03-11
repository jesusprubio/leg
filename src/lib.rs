/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */
use colored::*;

/// Create a custom logger.
///
/// * `tag` - String to use as prefix (after scope).
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - Use `eprintln` instead `eprint` (default: true).
pub fn custom(tag: &ColoredString, message: &str, scope: Option<&str>, ln: Option<bool>) {
    let pref = match scope {
        None => "".to_string(),
        Some(p) => format!("[{}]", p),
    };

    let to_print = format!("{} {} {}", pref.dimmed(), tag, message);

    match ln {
        Some(false) => eprint!("{}", to_print),
        _ => eprintln!("{}", to_print),
    }
}

/// Simple header/title for CLIs.
///
/// * `name` - Name of the project.
/// * `version` - Include also the version.
pub fn head(name: &str, icon: Option<&str>, version: Option<&str>) {
    let ver = match version {
        None => "".to_string(),
        Some(v) => format!("\n\t(v{})\n", v),
    };

    let ico = match icon {
        None => "",
        Some(i) => i,
    };

    eprintln!("\n\t{} {}{} ", ico, name.bold().underline(), ver.dimmed());
}

/// Informational message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn info(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"ℹ".blue().bold(), message, scope, ln);
}

/// Succesfull operation.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn success(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"✔".green().bold(), message, scope, ln);
}

/// Warn message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn warn(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"⚠".yellow().bold(), message, scope, ln);
}

/// Error message.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn error(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"✖".red().bold(), message, scope, ln);
}

/// Waiting for something.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn wait(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"…".magenta().bold(), message, scope, ln);
}

/// Something finished.
///
/// * `message` - String to print.
/// * `scope` - Preffix to append.
/// * `ln` - To print in the same line instead  (default: false).
pub fn done(message: &str, scope: Option<&str>, ln: Option<bool>) {
    custom(&"☒".cyan().bold(), message, scope, ln);
}

/// Put the cursor at the init of the actual line.
pub fn remove() {
    eprint!("\r");
}

/// Print the final result to the standard output.
///
/// * `message` - String to print.
pub fn result(message: &str) {
    eprintln!("{}", message);
}
