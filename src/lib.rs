pub mod components;
pub mod mdc_sys;

use std::sync::atomic::{AtomicU32, Ordering};
lazy_static::lazy_static! {
    static ref ID_COUNTER: AtomicU32 = AtomicU32::new(0);
}

// Tiny wrapper fn so we can make changes here instead of on every call
fn next_id() -> u32 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
