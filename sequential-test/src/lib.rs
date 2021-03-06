//! Allows for the creation of sequential tests.
//! ```ignore
//! #[cfg(test)]
//! mod tests {
//!     #[test]
//!     #[sequential]
//!     fn test1() {
//!         // ...
//!     }
//!     #[test]
//!     #[sequential]
//!     fn test2() {
//!         // ...
//!     }
//!     #[test]
//!     #[parallel]
//!     fn test3() {
//!         // ...
//!     }
//! }
//! ```
//! - Tests with the [`macro@sequential`] attribute are guaranteed to be executed sequentially.
//! - Tests with the [`macro@parallel`] attribute may run in parallel of each other but will not run
//! at the same time as tests with the [`macro@sequential`] attribute.
//! - Tests with neither attributes may run in parallel with any tests.
//!
//! Defining [`macro@sequential`] or [`macro@parallel`] attributes on non-tests or within scopes is
//! considered UB.
//!
//! This library is both faster[^speed] and smaller than
//! [`serial_test`](https://github.com/palfrey/serial_test) however offers less functionality.
//!
//! [^speed]: The current benchmark illustrate `sequential-test` covers the test set in an average
//! of ~350ms while [`serial_test`](https://github.com/palfrey/serial_test) covers the test set in
//! an average of ~550ms.

#[doc(hidden)]
pub use lazy_static::lazy_static;
pub use sequential_macro::*;
