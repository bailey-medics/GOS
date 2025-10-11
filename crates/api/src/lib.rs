#![doc = include_str!("index.md")]
#![warn(rust_2018_idioms)]

pub use service::{pb, VprService};

pub mod service;
