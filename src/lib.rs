// Copyright 2017 Thomas Schaller.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A small crate which allows combining
//! a [`Duration`]'s seconds and nanoseconds
//! into [seconds], [milliseconds] and [microseconds].
//! Additionally, it allows [easy formatting] of a
//! `Duration` for performance measurements.
//!
//! ## Conversion to fractional
//!
//! ```
//! # #![allow(unused_variables)]
//!
//! use std::time::Duration;
//! use floating_duration::TimeAsFloat;
//!
//! let duration = Duration::new(4, 123_456_789);
//!
//! let secs = duration.as_fractional_secs(); // 4.12..
//! let millis = duration.as_fractional_millis(); // 4_123.45..
//! let micros = duration.as_fractional_micros(); // 4_123_456.78..
//! ```
//!
//! ## Automatic formatting
//!
//! ```
//! use std::time::Instant;
//! use floating_duration::TimeFormat;
//!
//! # fn do_something() {}
//!
//! # fn main() {
//! let start = Instant::now();
//!
//! do_something();
//!
//! println!("Needed {}", TimeFormat(start.elapsed()));
//! # }
//! ```
//!
//! Output: `Needed 12.841µs`
//!
//! [`Duration`]: https://doc.rust-lang.org/stable/std/time/struct.Duration.html
//! [seconds]: trait.TimeAsFloat.html#tymethod.as_fractional_secs
//! [milliseconds]: trait.TimeAsFloat.html#tymethod.as_fractional_millis
//! [microseconds]: trait.TimeAsFloat.html#tymethod.as_fractional_micros
//!
//! [easy formatting]: struct.TimeFormat.html

use std::borrow::Borrow;
use std::fmt::{Display, Error as FormatError, Formatter};
use std::time::Duration;

/// Trait for providing `as_fractional_*` methods.
///
/// # Examples
///
/// ## Measuring a time span
///
/// ```
/// use std::time::Instant;
/// use floating_duration::TimeAsFloat;
///
/// let start = Instant::now();
///
/// let result = (1..12).fold(1, |acc, x| acc * x);
///
/// println!("Needed {} seconds", start.elapsed().as_fractional_secs());
/// // or:
/// println!("Needed {:#}", start.elapsed().as_fractional_secs()); // uses the full unit name
/// println!("Result: {}", result);
/// ```
pub trait TimeAsFloat {
    /// Returns the duration in seconds.
    fn as_fractional_secs(&self) -> f64;
    /// Returns the duration in milliseconds.
    fn as_fractional_millis(&self) -> f64;
    /// Returns the duration in microseconds.
    fn as_fractional_micros(&self) -> f64;
}

impl<T: Borrow<Duration>> TimeAsFloat for T {
    fn as_fractional_secs(&self) -> f64 {
        let dur: &Duration = self.borrow();

        dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0
    }

    fn as_fractional_millis(&self) -> f64 {
        let dur: &Duration = self.borrow();

        dur.as_secs() as f64 * 1_000.0 + dur.subsec_nanos() as f64 / 1_000_000.0
    }

    fn as_fractional_micros(&self) -> f64 {
        let dur: &Duration = self.borrow();

        dur.as_secs() as f64 * 1_000_000.0 + dur.subsec_nanos() as f64 / 1_000.0
    }
}

/// A formatting newtype for providing a
/// [`Display`] implementation. This format is
/// meant to be used for printing performance measurements.
///
/// # Behaviour
///
/// * `secs > 0` => seconds with up to 3 decimal places
/// * `secs > 0.001` => milliseconds with up to 3 decimal places
/// * `secs > 0.000_001` => microseconds with up to 3 decimal places
/// * otherwise => nanoseconds
///
/// By default the duration is formatted using abbreviated units
/// (e.g. `1.234ms`).
/// If the the format string is specified with the [alternate flag] `{:#}`,
/// the duration is formatted using the full unit name instead
/// (e.g. `1.234 milliseconds`).
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use floating_duration::TimeFormat;
///
/// let dur = Duration::new(0, 461_930);
/// let formatted = format!("{}", TimeFormat(dur));
/// assert_eq!(formatted, "461.93µs");
/// let alternate = format!("{:#}", TimeFormat(dur));
/// assert_eq!(alternate, "461.93 microseconds");
/// ```
///
/// [`Display`]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
/// [alternate flag]: https://doc.rust-lang.org/stable/std/fmt/#sign0
#[derive(Clone, Copy, Debug)]
pub struct TimeFormat<T: Borrow<Duration>>(pub T);

impl<T: Borrow<Duration>> Display for TimeFormat<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let dur: &Duration = self.0.borrow();

        if dur.as_secs() > 0 {
            if !f.alternate() {
                write!(f, "{}s", round_3_decimals(dur.as_fractional_secs()))
            } else {
                write!(f, "{} seconds", round_3_decimals(dur.as_fractional_secs()))
            }
        } else if dur.subsec_nanos() > 1_000_000 {
            if !f.alternate() {
                write!(f, "{}ms", round_3_decimals(dur.as_fractional_millis()))
            } else {
                write!(f, "{} milliseconds", round_3_decimals(dur.as_fractional_millis()))
            }
        } else if dur.subsec_nanos() > 1_000 {
            if !f.alternate() {
                write!(f, "{}µs", round_3_decimals(dur.as_fractional_micros()))
            } else {
                write!(f, "{} microseconds", round_3_decimals(dur.as_fractional_micros()))
            }
        } else {
            if !f.alternate() {
                write!(f, "{}ns", dur.subsec_nanos())
            } else {
                write!(f, "{} nanoseconds", dur.subsec_nanos())
            }
        }
    }
}

fn round_3_decimals(x: f64) -> f64 {
    (1000. * x).round() / 1000.
}
