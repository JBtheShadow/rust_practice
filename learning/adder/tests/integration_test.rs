// Integration tests go into the tests folder and don't require
// the #[cfg(test)] attribute
// Each file here is also its own crate so the library we're testing
// needs to be included each time
use adder;

// To create modules for use with tests, rather than the newer notation of files
// with [module_name].rs we use the old form [module_name_folder]/mod.rs
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// To run only tests from this file use
// cargo test --test integration_test