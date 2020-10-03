# heck-but-macros

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/heck-but-macros)
[![Crates.io](https://img.shields.io/crates/v/heck-but-macros)](https://crates.io/crates/heck-but-macros)
[![Docs.rs](https://docs.rs/heck-but-macros/badge.svg)](https://docs.rs/crates/heck-but-macros)

![Rust 1.40.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.40.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/heck-but-macros.svg?branch=develop)](https://travis-ci.com/Tamschi/heck-but-macros/branches)
![Crates.io - License](https://img.shields.io/crates/l/heck-but-macros/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/heck-but-macros)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/heck-but-macros)](https://github.com/Tamschi/heck-but-macros/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/heck-but-macros)](https://github.com/Tamschi/heck-but-macros/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/heck-but-macros.svg)](https://web.crev.dev/rust-reviews/crate/heck-but-macros/)

A proc macro wrapper around the [heck] crate, so that its casing functions can be applied to identifiers.

Also contains a workaround for (limited) stringification.

[heck]: https://github.com/withoutboats/heck

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add heck-but-macros
```

## Example

```rust
// TODO_EXAMPLE
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`heck-but-macros` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
