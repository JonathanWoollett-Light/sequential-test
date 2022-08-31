//! Allows for the creation of sequential tests.
//! ```
//! use sequential_test::sequential;
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

pub use sequential_macro::*;

#[cfg(test)]
mod tests {
    use super::{parallel, sequential};
    use rand::{thread_rng, Rng};
    use std::{thread, time::Duration};
    // use serial_test::{serial,parallel};

    const MAX_DURATION: Duration = Duration::from_millis(300);
    const FAILURE_RATE: u8 = 0; // 0..100
                                // Every test sleep for 0..MAX_DURATION then has a FAILURE_RATE % change to fail.
    fn test() {
        let mut rng = thread_rng();
        let duration = rng.gen_range(Duration::ZERO..MAX_DURATION);
        thread::sleep(duration);
        assert!(rng.gen_range(0..100) >= FAILURE_RATE);
    }
    /// Sequential test
    macro_rules! s {
        ($($i:ident),*) => {
            $(
                #[test]
                #[sequential]
                fn $i() {
                    test();
                }
            )*

        };
    }
    /// Parallel test
    macro_rules! p {
        ($($i:ident),*) => {
        $(
            #[test]
            #[parallel]
            fn $i() {
                test();
            }
        )*

    };
}
    s!(
        s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18, s19, s20,
        s21, s22, s23, s24, s25, s26, s27, s28, s29, s30, s31, s32, s33, s34, s35, s36, s37, s38,
        s39, s40
    );
    p!(
        p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20,
        p21, p22, p23, p24, p25, p26, p27, p28, p29, p30, p31, p32, p33, p34, p35, p36, p37, p38,
        p39, p40
    );
}
