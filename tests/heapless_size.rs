//! Regression guard for issue #2: in `heapless` mode the bounded
//! capacities are inline storage, so `size_of::<Dbc>()` *is* the memory
//! story. The old defaults made it gigabytes (unlinkable debug objects);
//! keep it small enough to live on a test thread's stack and in an
//! embedded static without ceremony.

#![cfg(all(feature = "heapless", not(feature = "alloc")))]

#[test]
fn dbc_by_value_stays_stack_sized() {
    let size = core::mem::size_of::<dbc_rs::Dbc>();
    println!("size_of::<Dbc>() = {size} bytes ({} KiB)", size / 1024);
    assert!(
        size <= 256 * 1024,
        "Dbc is {size} bytes — heapless defaults have regressed (issue #2)"
    );
}
