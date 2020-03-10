/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

#[cfg(test)]
use leg::*;

#[test]
fn should_work() {
    head("leg", Some("🔈"), Some("1.0.0"));

    info("Informational message", None, None);
    info("Informational message with scope", Some("myscope"), None);
    info("Informational message without new line", None, Some(false));
    info(
        "Informational message with scope without new line",
        Some("myscope"),
        Some(false),
    );

    success("Succesfull operation", None, None);
    success("Succesfull operation with scope", Some("myscope"), None);
    success("Succesfull operation without new line", None, Some(false));
    success(
        "Succesfull operation with scope without new line",
        Some("myscope"),
        Some(false),
    );

    warn("Warn message", None, None);
    warn("Warn message with scope", Some("myscope"), None);
    warn("Warn message without new line", None, Some(false));
    warn(
        "Warn message with scope without new line",
        Some("myscope"),
        Some(false),
    );

    error("Error message", None, None);
    error("Error message with scope", Some("myscope"), None);
    error("Error message without new line", None, Some(false));
    error(
        "Error message with scope without new line",
        Some("myscope"),
        Some(false),
    );

    wait("Waiting for something", None, None);
    wait("Waiting for something with scope", Some("myscope"), None);
    wait("Waiting for something without new line", None, Some(false));
    wait(
        "Waiting for something with scope without new line",
        Some("myscope"),
        Some(false),
    );

    done("Something finished", None, None);
    done("Something finished with scope", Some("myscope"), None);
    done("Something finished without new line", None, Some(false));
    done(
        "Something finished with scope without new line",
        Some("myscope"),
        Some(false),
    );

    eprint!("Not shown");
    remove();
}
