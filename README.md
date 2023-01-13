# rspy

Pythonic interface for Rust.

[![Crates.io](https://img.shields.io/crates/v/rspy)](https://crates.io/crates/rspy)
[![Crates.io](https://img.shields.io/crates/d/rspy)](https://crates.io/crates/rspy)
[![cargo test](https://github.com/seijinrosen/rspy/actions/workflows/cargo_test.yml/badge.svg)](https://github.com/seijinrosen/rspy/actions/workflows/cargo_test.yml)
[![codecov](https://codecov.io/gh/seijinrosen/rspy/branch/main/graph/badge.svg)](https://codecov.io/gh/seijinrosen/rspy)

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
rspy = "0"
```

## Currently available

| Python                   | Rust                            |
| ------------------------ | ------------------------------- |
| [builtins.enumerate]     | rspy::Enumerator::enumerate     |
| [builtins.input]         | [rspy::input]                   |
| [string.ascii_lowercase] | [rspy::string::ASCII_LOWERCASE] |
| [string.ascii_uppercase] | [rspy::string::ASCII_UPPERCASE] |

## TODO

- [pathlib.Path.mkdir]

[builtins.enumerate]: https://docs.python.org/ja/3/library/functions.html#enumerate
[builtins.input]: https://docs.python.org/ja/3/library/functions.html#input
[pathlib.path.mkdir]: https://docs.python.org/ja/3/library/pathlib.html#pathlib.Path.mkdir
[string.ascii_lowercase]: https://docs.python.org/ja/3/library/string.html#string.ascii_lowercase
[string.ascii_uppercase]: https://docs.python.org/ja/3/library/string.html#string.ascii_uppercase
[rspy::input]: https://docs.rs/rspy/latest/rspy/fn.input.html
[rspy::string::ascii_lowercase]: https://docs.rs/rspy/latest/rspy/string/constant.ASCII_LOWERCASE.html
[rspy::string::ascii_uppercase]: https://docs.rs/rspy/latest/rspy/string/constant.ASCII_UPPERCASE.html
