/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */

#[cfg(test)]
use async_std::eprint;
use leg::*;

#[async_std::test]
async fn should_work() {
    head("leg", Some("🔈"), Some("1.0.0")).await;

    eprint!("Not shown").await;
    remove().await;

    info("Informational message", None, None).await;
    info("Informational message with scope", Some("myscope"), None).await;
    info("Informational message without new line", None, Some(false)).await;
    info(
        "Informational message with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    success("Succesfull operation", None, None).await;
    success("Succesfull operation with scope", Some("myscope"), None).await;
    success("Succesfull operation without new line", None, Some(false)).await;
    success(
        "Succesfull operation with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    warn("Warn message", None, None).await;
    warn("Warn message with scope", Some("myscope"), None).await;
    warn("Warn message without new line", None, Some(false)).await;
    warn(
        "Warn message with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    error("Error message", None, None).await;
    error("Error message with scope", Some("myscope"), None).await;
    error("Error message without new line", None, Some(false)).await;
    error(
        "Error message with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    wait("Waiting for something", None, None).await;
    wait("Waiting for something with scope", Some("myscope"), None).await;
    wait("Waiting for something without new line", None, Some(false)).await;
    wait(
        "Waiting for something with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    done("Something finished", None, None).await;
    done("Something finished with scope", Some("myscope"), None).await;
    done("Something finished without new line", None, Some(false)).await;
    done(
        "Something finished with scope without new line",
        Some("myscope"),
        Some(false),
    )
    .await;

    result("Result printed to the standard output").await;
}
