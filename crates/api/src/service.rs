// Re-export the proto module from the shared `vpr-proto` crate so callers
// can continue to reference `api::service::pb`.
pub use vpr_proto::pb;

// Re-export the service implementation type directly from the `vpr-temp` crate.
// This ensures the type is publicly available as `api::service::VprService`.
pub use vpr_temp::VprService;
