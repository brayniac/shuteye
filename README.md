# shuteye - nanosleep for rust

shuteye provides for high-resolution sleep in rust. Let your code catch some shuteye

The API documentation of this library can be found at
[docs.rs/shuteye](https://docs.rs/shuteye/).

[![conduct-badge][]][conduct] [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[travis-badge]: https://img.shields.io/travis/brayniac/shuteye/master.svg
[downloads-badge]: https://img.shields.io/crates/d/shuteye.svg
[release-badge]: https://img.shields.io/crates/v/shuteye.svg
[license-badge]: https://img.shields.io/crates/l/shuteye.svg
[conduct]: https://brayniac.github.io/conduct
[travis]: https://travis-ci.org/brayniac/shuteye
[crate]: https://crates.io/crates/shuteye
[Cargo]: https://github.com/rust-lang/cargo

## Code of Conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct]

## Usage

To use `shuteye`, first add this to your `Cargo.toml`:

```toml
[dependencies]
shuteye = "^0"
```

Then, add this to your crate root:

```rust
extern crate shuteye;
```

## Documentation

View the docs here: [https://docs.rs/shuteye/](https://docs.rs/shuteye/)

## Features

* nanosleep

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
