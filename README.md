# leg

🔈 **Nicer CLIs foz lazy devs**

<img
  width="344" alt="Screenshot 2023-03-06 at 14 18 25"
  src="https://user-images.githubusercontent.com/2753855/223247609-2ab48185-b26f-4763-ba33-ecc9bdc6f07a.png"
>

[![GoDoc][doc-img]][doc] [![Build Status][ci-img]][ci]

Tiny wrapper over go printtttt helpers to beauty my command line application
prototypes with minimal effort.

- Prints to `stderr`.
- Thanks to [color](https://github.com/fatih/color) library:
  - Cross platform.
  - Respects [NO_COLOR](https://no-color.org).

## Install

```sg
go get -u github.com/jesusprubio/leg
```

### Dependencies

- [Go](https://go.dev/doc/install) stable version.

## Use

Visit the [tests](leg_test.go) to check more details.

```golang
leg.Head("mumu", "🐘", "0.1.0")
leg.Info("Informational message", "")
leg.Success("Successful operation", "")
leg.Warn("Warn message", "")
leg.Error("Error message", "")
leg.Wait("Waiting for something", "")
leg.Done("Something finished", "")

leg.Info("Informational message with scope", "scope-0")

fmt.Print("deleted line")
leg.Remove()

leg.Result("To standard output")
```

[doc-img]: https://pkg.go.dev/badge/github.com/jesusprubio/leg
[doc]: https://pkg.go.dev/github.com/jesusprubio/leg
[ci-img]: https://github.com/jesusprubio/leg/workflows/CI/badge.svg
[ci]: https://github.com/jesusprubio/leg/workflows/go.yml
