#![doc = include_str!("index.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(html_logo_url = "https://www.rust-lang.org/lovpr/rust-logo-128x128-blk.png")]
#![doc(html_favicon_url = "https://www.rust-lang.org/favicon.ico")]
#![warn(rust_2018_idioms)]

pub use service::{pb, VprService};

pub mod service;
