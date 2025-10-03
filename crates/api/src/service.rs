// Re-export the proto module from the shared `gos-proto` crate so callers
// can continue to reference `api::service::pb`.
pub use gos_proto::pb;

// Re-export the service implementation type directly from the `gitehr-temp` crate.
// This ensures the type is publicly available as `api::service::GosService`.
pub use gitehr_temp::GosService;
