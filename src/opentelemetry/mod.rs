//! openelemetry module re-exports
//!
//! This module re-exports the crates supported and used by rama for (open) telemetry,
//! such that you can make use of it for custom metrics, registries and more.

pub use ::opentelemetry::*;

pub use ::opentelemetry_semantic_conventions as semantic_conventions;

pub use ::opentelemetry_sdk as sdk;

pub mod prometheus {
    //! prometheus module re-exports

    pub use ::opentelemetry_prometheus::*;
    pub use ::prometheus::*;
}
