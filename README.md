<h1 align="center">leg</h1>

<h4 align="center">
  🔈 Elegant logging for CLIs made simple
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

- Prints to `stderr` to be nice with your pipes.
- Thanks to the awesome [colored](https://crates.io/crates/colored) crate:
  - Works on Linux, MacOS, and Windows (Powershell).
  - Respects [CLICOLOR/CLICOLOR_FORCE](http://bixense.com/clicolors) and [NO_COLOR](https://no-color.org) behavior.

**Do not use this crate in libraries! You should use the [log](https://github.com/rust-lang-nursery/log) one instead.**

## Use

📝 Please visit [tests](tests/lib.rs) and [full documentation](https://docs.rs/leg) if you want to learn the details.

```rust
use leg::*;

head("leg", Some("🔈"), Some("1.0.0"));

info("Informational message", None, None);
success("Successful operation", None, None);
warn("Warn message", None, None);
error("Error message", None, None);
wait("Waiting for something", None, None);
done("Something finished", None, None);

info("Informational message with scope", Some("myscope"), None);
info(
    "Informational message without new line",
    None,
    Some(false),
);
eprintln!(" => same line");

eprint!("Not shown");
remove();

result("To standard output")
```

## Contributing

😎 If you want to help please take a look to [this file](.github/CONTRIBUTING.md).
