# Minimize All Windows

## Environment

```powershell
PS C:\Users\owner> [System.Environment]::OSVersion.Version

Major  Minor  Build  Revision
-----  -----  -----  --------
10     0      22631  0


PS C:\Users\owner> rustc -V
rustc 1.87.0-nightly (794c12416 2025-02-21)
```

## Note

### Features

This project includes the following features:

- `reopen-cookie-clicker`: Automatically restores the Cookie Clicker window after minimizing all windows
- `shut-off-monitor`: Turns off the monitor after minimizing all windows

To enable all features, use:

```shell
cargo run --features "reopen-cookie-clicker"
cargo run --features "shut-off-monitor"
cargo run --all-features
```
