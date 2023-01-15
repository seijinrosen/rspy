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

| Python                   | Rust                                    |
| ------------------------ | --------------------------------------- |
| [builtins.enumerate]     | [rspy::Iterable::enumerate] (trait way) |
|                          | [rspy::enumerate] (function way)        |
| [builtins.input]         | [rspy::input]                           |
| [builtins.sorted]        | rspy::Iterable::sorted (trait way)      |
| [pathlib.Path.mkdir]     | [rspy::pathlib::PyPath::mkdir]          |
| [string.ascii_lowercase] | [rspy::string::ASCII_LOWERCASE]         |
| [string.ascii_uppercase] | [rspy::string::ASCII_UPPERCASE]         |
| [time.sleep]             | [rspy::time::sleep]                     |

[builtins.enumerate]: https://docs.python.org/ja/3/library/functions.html#enumerate
[builtins.input]: https://docs.python.org/ja/3/library/functions.html#input
[builtins.sorted]: https://docs.python.org/ja/3/library/functions.html#sorted
[pathlib.path.mkdir]: https://docs.python.org/ja/3/library/pathlib.html#pathlib.Path.mkdir
[string.ascii_lowercase]: https://docs.python.org/ja/3/library/string.html#string.ascii_lowercase
[string.ascii_uppercase]: https://docs.python.org/ja/3/library/string.html#string.ascii_uppercase
[time.sleep]: https://docs.python.org/ja/3/library/time.html#time.sleep
[rspy::iterable::enumerate]: https://docs.rs/rspy/latest/rspy/trait.Iterable.html#tymethod.enumerate
[rspy::enumerate]: https://docs.rs/rspy/latest/rspy/fn.enumerate.html
[rspy::input]: https://docs.rs/rspy/latest/rspy/fn.input.html
[rspy::pathlib::pypath::mkdir]: https://docs.rs/rspy/latest/rspy/pathlib/trait.PyPath.html#tymethod.mkdir
[rspy::string::ascii_lowercase]: https://docs.rs/rspy/latest/rspy/string/constant.ASCII_LOWERCASE.html
[rspy::string::ascii_uppercase]: https://docs.rs/rspy/latest/rspy/string/constant.ASCII_UPPERCASE.html
[rspy::time::sleep]: https://docs.rs/rspy/latest/rspy/time/fn.sleep.html
