#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

// Phase 97.3.esp32-qemu — when the `tracing` Cargo feature is off
// (e.g. `riscv32imc` ESP32-C3 builds where `tracing-core`'s
// `core::sync::atomic::*::compare_exchange` calls won't compile),
// alias the dust_dds crate itself as `tracing` so `use tracing::warn;`
// + `tracing::warn!(...)` resolve to no-op macros re-exported here.
// `#[tracing::instrument(...)]` attributes are separately wrapped in
// `cfg_attr(feature = "tracing", ...)` so they expand to nothing.
//
// With the feature on, none of this compiles and `tracing` resolves
// to the real upstream crate as before.
#[cfg(not(feature = "tracing"))]
extern crate self as tracing;

#[cfg(not(feature = "tracing"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __dustdds_tracing_noop {
    ($($tt:tt)*) => {{}};
}

#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_noop as debug;
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_noop as error;
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_noop as info;
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_noop as trace;
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_noop as warn;

// span! returns a stub with .enter() that returns a stub guard.
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub struct __DustddsTracingSpan;

#[cfg(not(feature = "tracing"))]
impl __DustddsTracingSpan {
    #[doc(hidden)]
    pub fn enter(&self) -> __DustddsTracingSpanGuard {
        __DustddsTracingSpanGuard
    }
}

#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub struct __DustddsTracingSpanGuard;

#[cfg(not(feature = "tracing"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __dustdds_tracing_span_noop {
    ($($tt:tt)*) => {{ $crate::__DustddsTracingSpan }};
}

#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
pub use crate::__dustdds_tracing_span_noop as span;

// Level stub so `tracing::Level::INFO` resolves.
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod Level {
    #[doc(hidden)]
    pub const INFO: () = ();
    #[doc(hidden)]
    pub const DEBUG: () = ();
    #[doc(hidden)]
    pub const TRACE: () = ();
    #[doc(hidden)]
    pub const WARN: () = ();
    #[doc(hidden)]
    pub const ERROR: () = ();
}

#[cfg(feature = "std")]
mod dds;

#[cfg(feature = "std")]
pub use dds::*;

/// Contains the async version of the DDS API.
#[cfg(feature = "dcps")]
pub mod dds_async;

/// Contains the DCPS logic which provides the behavior to the DDS API
#[doc(hidden)]
#[cfg(feature = "dcps")]
pub mod dcps;

#[cfg(feature = "dcps")]
pub use dcps::{builtin_topics, infrastructure};

#[doc(hidden)]
#[cfg(feature = "rtps")]
pub mod rtps;

#[cfg(feature = "rtps_messages")]
#[doc(hidden)]
pub mod rtps_messages;

#[cfg(feature = "rtps_udp_transport")]
#[doc(hidden)]
pub mod rtps_udp_transport;

#[cfg(feature = "transport")]
#[doc(hidden)]
/// Contains the Dust DDS transport interface definition.
pub mod transport;

#[cfg(feature = "dcps")]
/// Contains the Dust DDS runtime abstractions.
pub mod runtime;

#[cfg(feature = "std")]
#[doc(hidden)]
pub mod std_runtime;

/// Contains the XTypes serializer and deserializer
#[cfg(feature = "xtypes")]
#[doc(hidden)]
pub mod xtypes;

// To enable using our own derive macros to allow the name dust_dds:: to be used
extern crate self as dust_dds;
