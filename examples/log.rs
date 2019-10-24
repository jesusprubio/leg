/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *                     Hikaru Terazono <3c1u@vulpesgames.tokyo>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

use leg::*;

#[cfg(not(feature = "log_crate"))]
fn main() {
}

#[cfg(feature = "log_crate")]
fn main() {
    use log::*;

    // First, you need to set the logger level.
    //
    // e.g.)
    // $ export RUST_LOG=trace

    // Then initialise the logger by calling leg::init().
    leg::init();

    head("leg", Some("🔈"), Some("1.0.0"));

    // Now that you can use log macros.
    trace!("trace");
    info!("info");
    debug!("debug");
    warn!("warn");
    error!("error");
}
