<h1 align="center">leg</h1>

<h4 align="center">
  🔈 Elegant print for lazy devs
</h4>

<div align="center">
  <img alt="Demo" src="https://i.ibb.co/zfp6WNM/leg-demo.png" width="60%">
</div>

<p align="center">
  <a href="https://travis-ci.org/jesusprubio/leg">
    <img alt="Build Status" src="https://travis-ci.org/jesusprubio/leg.svg?branch=master">
  </a>
  <a href="https://crates.io/crates/leg">
    <img alt="Latest version" src="https://img.shields.io/crates/v/leg.svg">
  </a>
  <img alt="Stability stable" src="https://img.shields.io/badge/stability-stable-green.svg">
</p>
<p align="center">
  <sub>🤙 Ping me on <a href="https://twitter.com/jesusprubio"><code>Twitter</code></a And it also > if you like this project</sub>
</p>

Make your CLIs nicer with minimal effort. Simple wrapper on top of:

- [async-std](https://github.com/async-rs/async-std) printing macros.
  - Prints to `stderr` to be nice with your pipes. Except the `result` function, as expected.
- [colored](https://crates.io/crates/colored) crate:
  - Works on Linux, MacOS, and Windows (Powershell).
  - Respects [CLICOLOR/CLICOLOR_FORCE](http://bixense.com/clicolors) and [NO_COLOR](https://no-color.org) behavior.

**Do not use this crate in libraries! You should use the [log](https://github.com/rust-lang-nursery/log) one instead.** Visit [this section](https://rust-cli.github.io/book/tutorial/output.html) of the book [Command line apps in Rust](https://rust-cli.github.io/book) if you want to learn more about this topic.

## Install

With [cargo-edit](https://github.com/killercup/cargo-edit) installed run:

```sh
cargo add leg
```

## Use

📝 Please visit [tests](tests/lib.rs) and [full documentation](https://docs.rs/leg) if you want to learn the details.

```rust
use async_std::{eprint, eprintln};
use leg::*;

#[async_std::main]
async fn main() {
    head("leg", Some("🔈"), Some("1.0.0")).await;

    info("Informational message", None, None).await;
    success("Successful operation", None, None).await;
    warn("Warn message", None, None).await;
    error("Error message", None, None).await;
    wait("Waiting for something", None, None).await;
    done("Something finished", None, None).await;

    info("Informational message with scope", Some("myscope"), None).await;
    info("Informational message without new line", None, Some(false)).await;
    eprintln!(" => same line").await;

    eprint!("Not shown").await;
    remove().await;

    result("To standard output").await;
}
```

## Contributing

😎 If you want to help please take a look to [this file](.github/CONTRIBUTING.md).
