//! **Do not use this**
#![warn(clippy::pedantic)]
extern crate proc_macro;

use quote::quote;
use std::sync::atomic::{AtomicBool, Ordering};

/// Used for the attribute macros such that we only define [`__TestState`] and [`__PAIR`] once.
static SET: AtomicBool = AtomicBool::new(false);
/// Annotates tests which must run sequentially.
#[proc_macro_attribute]
pub fn sequential(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    inner(
        item,
        quote! {
            let _ = __PAIR.1.wait_while(__PAIR.0.lock().expect("sequential-test error"), |pending|
                match pending {
                    __TestState::Parallel(0) => {
                        *pending = __TestState::Sequential;
                        false
                    },
                    _ => true
                }
            ).expect("sequential-test error");
        },
        quote! {
            *__PAIR.0.lock().expect("sequential-test error") = __TestState::Parallel(0);
        },
    )
}
/// Annotates tests which may run in parallel.
#[proc_macro_attribute]
pub fn parallel(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    inner(
        item,
        quote! {
            let _ = __PAIR.1.wait_while(__PAIR.0.lock().expect("sequential-test error"), |pending|
                match pending {
                    __TestState::Sequential => true,
                    __TestState::Parallel(ref mut x) => {
                        *x += 1;
                        false
                    }
                }
            ).expect("sequential-test error");
        },
        quote! {
            match *__PAIR.0.lock().expect("sequential-test error") {
                __TestState::Sequential => unreachable!("sequential-test error"),
                __TestState::Parallel(ref mut x) => {
                    *x -= 1;
                }
            }
        },
    )
}
fn inner(
    item: proc_macro::TokenStream,
    prefix: proc_macro2::TokenStream,
    suffix: proc_macro2::TokenStream,
) -> proc_macro::TokenStream {
    // We get whether this was the first macro invocation and set first macro invocation to false.
    let ret = SET.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
    let mut iter = item.into_iter().peekable();
    let signature = proc_macro2::TokenStream::from(
        std::iter::from_fn(|| {
            iter.next_if(|x| match x {
                proc_macro::TokenTree::Group(group) => {
                    !matches!(group.delimiter(), proc_macro::Delimiter::Brace)
                }
                _ => true,
            })
        })
        .collect::<proc_macro::TokenStream>(),
    );
    let block = proc_macro2::TokenStream::from(iter.collect::<proc_macro::TokenStream>());
    let item = quote! {
        #signature {
            #prefix
            let res = std::panic::catch_unwind(|| #block );
            #suffix
            __PAIR.1.notify_all();
            if let Err(err) = res {
                std::panic::resume_unwind(err);
            }
        }
    };
    // If this was the first macro invocation define the mutex and condvar used for locking.
    let rtn = if ret.is_ok() {
        quote! {
            /// The test state stored in the mutex in the pair used for ordering tests.
            enum __TestState {
                Sequential,
                Parallel(u64)
            }
            /// The mutex and condvar pair used for ordering tests.
            static __PAIR: (std::sync::Mutex<__TestState>, std::sync::Condvar) = (
                std::sync::Mutex::new(__TestState::Parallel(0)), std::sync::Condvar::new()
            );
            #item
        }
    } else {
        item
    };
    proc_macro::TokenStream::from(rtn)
}
