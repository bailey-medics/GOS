#![doc = include_str!("index.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]
#![doc(html_favicon_url = "https://www.rust-lang.org/favicon.ico")]
#![warn(rust_2018_idioms)]
// Note: We don't warn about missing_docs globally to avoid issues with generated protobuf code

// Re-export the main service and protobuf definitions
pub use service::{pb, GosService};

/// Service implementation module containing the gRPC service logic.
pub mod service;
