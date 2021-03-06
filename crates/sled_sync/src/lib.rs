#[cfg(any(test, feature = "lock_free_delays"))]
mod debug_delay;

#[cfg(any(test, feature = "lock_free_delays"))]
pub use self::debug_delay::debug_delay;

/// This function is useful for inducing random jitter into our atomic
/// operations, shaking out more possible interleavings quickly. It gets
/// fully elliminated by the compiler in non-test code.
#[cfg(not(any(test, feature = "lock_free_delays")))]
pub fn debug_delay() {}

pub use crossbeam_epoch::{
    pin, unprotected, Atomic, Collector, CompareAndSetError, Guard,
    LocalHandle, Owned, Shared,
};

pub use crossbeam_utils::Backoff;

pub use std::sync::atomic;
