/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@member.fsf.org>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

use leg::*;

fn main() {
    head("leg", Some("🔈"), Some("1.0.0"));

    info("Informational message", None, None);
    success("Succesfull operation", None, None);
    warn("Warn message", None, None);
    error("Error message", None, None);
    wait("Waiting for something", None, None);
    done("Something finished", None, None);

    print!("Not shown");
    remove();

    info("Informational message with scope", Some("myscope"), None);
    info("Informational message without new line", None, Some(false));
    println!(" => same line");
}
