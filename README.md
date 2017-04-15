# floating-duration

[![Build Status][bi]][bl] [![Crates.io][ci]][cl] ![MIT/Apache][li] [![Docs.rs][di]][dl]

[bi]: https://travis-ci.org/torkleyy/floating-duration.svg?branch=master
[bl]: https://travis-ci.org/torkleyy/floating-duration

[ci]: https://img.shields.io/crates/v/floating-duration.svg
[cl]: https://crates.io/crates/floating-duration/

[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg

[di]: https://docs.rs/floating-duration/badge.svg
[dl]: https://docs.rs/floating-duration/

Allows converting a `Duration` to floating-point seconds, milliseconds and microseconds. 
Additionally, it allows automatic formatting of a `Duration` (it automatically chooses
a unit).

## Usage

Minimum Rust version: `1.8.0`

Add this crate to `Cargo.toml`

```toml
[dependencies]
floating-duration = "0.1"
```

Now you can easily print a `Duration`:

```rust
extern crate floating_duration;

use std::time::Instant;

use floating_duration::{TimeAsFloat, TimeFormat};

fn main() {
    let start = Instant::now();
    
    let result = (1..12).fold(1, |acc, x| acc * x);
    
    println!("Needed {}", TimeFormat(start.elapsed()));
    println!("In seconds: {}", start.elapsed().as_fractional_secs());
}
```

## Contribution

Contribution is very welcome!

Any contribution you submit is assumed to be
dual-licensed under MIT/Apache-2.

## License

floating-duration is distributed under the terms of both the MIT license 
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
