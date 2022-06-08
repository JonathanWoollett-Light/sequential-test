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
//! This library does not support `async` tests.
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
#![warn(clippy::pedantic)]
extern crate proc_macro;
use proc_macro::TokenStream;

use std::sync::atomic::{AtomicBool, Ordering};

/// Used for tje attribute macros such that we only define [`__TestState`] and [`__PAIR`] once.
static SET: AtomicBool = AtomicBool::new(false);

const DEFINE: &str = "
    /// The test state stored in the mutex in the pair used for ordering tests.
    enum __TestState {
        Sequential,
        Parallel(u64)
    }
    /// The mutex and condvar pair used for ordering tests.
    lazy_static::lazy_static! {
        static ref __PAIR: std::sync::Arc<(std::sync::Mutex<__TestState>, std::sync::Condvar)> = 
            std::sync::Arc::new(
                (std::sync::Mutex::new(__TestState::Parallel(0)), std::sync::Condvar::new())
            );
    }
";
const SEQ_PREFIX: &str = "
    let _ = __PAIR.1.wait_while(__PAIR.0.lock().unwrap(), |pending| 
        match pending {
            __TestState::Parallel(0) => {
                *pending = __TestState::Sequential;
                false
            },
            _ => true
        }
    ).unwrap();
";
const PAR_PREFIX: &str = "
    let _ = __PAIR.1.wait_while(__PAIR.0.lock().unwrap(), |pending|
        match pending {
            __TestState::Sequential => true,
            __TestState::Parallel(ref mut x) => {
                *x += 1;
                false
            }
        }
    ).unwrap();
";
const PAR_SUFFIX: &str = "
    match *__PAIR.0.lock().unwrap() {
        __TestState::Sequential => unreachable!(),
        __TestState::Parallel(ref mut x) => {
            *x -= 1;
        }
    }
";
const SEQ_SUFFIX: &str = "*__PAIR.0.lock().unwrap() = __TestState::Parallel(0);";
const SUFFIX: &str = "
    __PAIR.1.notify_all();
    if let Err(err) = res {
        std::panic::resume_unwind(err);
    }
";

/// Annotates tests which must run sequentially.
#[proc_macro_attribute]
pub fn sequential(_attr: TokenStream, item: TokenStream) -> TokenStream {
    inner(item, SEQ_PREFIX, SEQ_SUFFIX)
}
/// Annotates tests which may run in parallel.
#[proc_macro_attribute]
pub fn parallel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    inner(item, PAR_PREFIX, PAR_SUFFIX)
}
fn inner(item: TokenStream, prefix: &str, suffix: &str) -> TokenStream {
    let ret = SET.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
    let mut iter = item.into_iter();
    let signature = (&mut iter).take(3).collect::<TokenStream>();
    let block = iter.collect::<TokenStream>();

    let item = format!(
        "
        {} {{
            {}
            let res = std::panic::catch_unwind(|| {} );
            {}
            {}
        }}",
        signature, prefix, block, suffix, SUFFIX
    );
    if ret.is_ok() {
        format!("{}\n{}", DEFINE, item)
    } else {
        item
    }
    .parse()
    .unwrap()
}
