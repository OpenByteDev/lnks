# lnks

[![CI](https://github.com/OpenByteDev/lnks/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/lnks/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/lnks.svg)](https://crates.io/crates/lnks) [![Documentation](https://docs.rs/lnks/badge.svg)](https://docs.rs/lnks) [![dependency status](https://deps.rs/repo/github/openbytedev/lnks/status.svg)](https://deps.rs/repo/github/openbytedev/lnks) [![MIT](https://img.shields.io/crates/l/lnks.svg)](https://github.com/OpenByteDev/lnks/blob/master/LICENSE)

`lnks` provides a high-level API for reading and writing Windows `.lnk` (Shell Link) files.
It wraps the COM-based Shell APIs `IShellLinkW` and `IPersistFile`, including support for reading and toggling the undocumented "Run as administrator" flag.

## Examples

### Load an existing shortcut
```rust
let path = Path::new(r"C:\Users\Public\Desktop\Notepad.lnk");
let shortcut = lnks::Shortcut::load(path).unwrap();
```

### Create a new shortcut
```rust
let mut shortcut = lnks::Shortcut::new(r"C:\Windows\System32\notepad.exe");
shortcut.arguments = Some(r"C:\Windows\win.ini".to_string());
let out = Path::new(r"C:\Users\Public\Desktop\Notepad.lnk");
shortcut.save(out).unwrap();
```

## License
Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/lnks/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
