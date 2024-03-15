# Developer Guide

## Dependencies

- [Task](https://taskfile.dev/installation/)
- Linters:
```sh
task dep
```

### Tests

To be sure the new code makes the linters happy and the tests are not broken.

```sh
task # Alias for the next ones.
task lint
task test
task format
```
