# sequential-test

[![Version](https://img.shields.io/crates/v/sequential-test.svg)](https://crates.io/crates/sequential-test)
[![Downloads](https://img.shields.io/crates/d/sequential-test)](https://crates.io/crates/sequential-test)
[![Docs](https://docs.rs/sequential-test/badge.svg)](https://docs.rs/sequential-test/)
[![APACHE 2.0 license](https://img.shields.io/crates/l/sequential-test.svg)](./LICENSE)

*If you want a more fully featured mature implementation checkout [palfrey/serial_test](https://github.com/palfrey/serial_test)*

Allows for the creation of sequential tests.
```rust
#[cfg(test)]
mod tests {
    #[test]
    #[sequential]
    fn test1() {
        // ...
    }
    #[test]
    #[sequential]
    fn test2() {
        // ...
    }
    #[test]
    #[parallel]
    fn test3() {
        // ...
    }
}
```
- Tests with the `sequential` attribute are guaranteed to be executed sequentially.
- Tests with the `parallel` attribute may run in parallel of each other but will not run
at the same time as tests with the `sequential` attribute.
- Tests with neither attributes may run in parallel with any tests.

Defining `sequential` or `parallel` attributes on non-tests or within scopes is
considered UB.

This library is faster[^speed] and smaller but less tested and younger than
[`serial_test`](https://github.com/palfrey/serial_test).

[^speed]: The current benchmark illustrate `sequential-test` covers the test set in an average 
of ~350ms while [`serial_test`](https://github.com/palfrey/serial_test) covers the test set in 
an average of ~550ms.
