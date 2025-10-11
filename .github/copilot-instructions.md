<!--
Guidance for AI coding agents working on the VPR repository.
Focus: be pragmatic, reference concrete files, and keep changes minimal and well-tested.
-->
# VPR — AI contributor notes

These notes are for automated coding agents and should be short, concrete, and codebase-specific.

Overview
- Purpose: VPR is a small Rust-based gRPC service (health + patient creation) built as a Cargo workspace. The primary service binary is in the `api` crate and exposes a `VPR` gRPC service (see `crates/api/proto/vpr/v1/vpr.proto`).
- Key crates:
  - `crates/api` — gRPC server binary (uses `tonic`, entry at `crates/api/src/main.rs`).
  - `crates/vpr-proto` — protobuf generation (build script produces `pb` module; see `crates/vpr-proto/src/lib.rs`).
  - `crates/vpr` / `crates/vpr-temp` — service implementation types re-exported by `api` (see `crates/api/src/service.rs`).

Important files to reference
- `README.md` — high-level project description and docs link.
- `Justfile` — developer convenience commands (aliases for start-dev, docs, pre-commit). Use `just <target>` where helpful.
- `compose.dev.yml` — development Docker setup; useful example invocations and healthcheck command (`grpcurl -plaintext localhost:50051 list`).
- `proto/` and `crates/api/proto/vpr/v1/vpr.proto` — canonical protobuf definitions. Follow the exact package and message names when changing RPCs.
- `scripts/` — useful wrappers: `fmt.sh`, `lint.sh`, `check-all.sh` (format, clippy, check, test).

Build and test workflows (concrete)
- Local quick compile of the API binary:
  - `cargo run -p api` (or `cargo run -p api --bin vpr-api` if ambiguous)
- Full workspace checks (used by CI and `scripts/check-all.sh`):
  - `./scripts/check-all.sh` — runs `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, and `cargo test`.
- Docker dev runtime:
  - `docker compose -f compose.dev.yml up --build` or use `just start-dev b`.
  - Healthcheck uses `grpcurl -plaintext localhost:50051 list` inside container.

Conventions and patterns to follow
- Protobufs: the canonical proto lives at `crates/api/proto/vpr/v1/vpr.proto`. Generated Rust modules are provided by `crates/vpr-proto` and included via `tonic::include_proto!("vpr.v1")`.
- Service wiring: `crates/api` re-exports proto and service types so callers use `api::pb` and `api::VprService`.
- Logging: uses `tracing`/`tracing-subscriber`. Default env var: `VPR_ADDR` and `RUST_LOG` control runtime behaviour.
- File-based patient persistence: `CreatePatient` returns a filename and writes JSON under `patient_data/` — be careful when changing storage paths; Docker mounts `./patient_data` for persistence.

Change policy and safety
- Prefer minimal, well-scoped PRs that update a single crate or module.
- Run `./scripts/check-all.sh` before proposing changes. Fix any clippy or formatting issues locally.
- When changing protos: update `crates/api/proto/...`, then ensure the build script in `crates/vpr-proto` regenerates code. Run a workspace build to confirm no downstream breakage.

Examples (copyable snippets)
- Start the server locally on the default port:
  - `VPR_ADDR=0.0.0.0:50051 cargo run -p api`
- List RPCs with grpcurl inside dev container:
  - `docker compose -f compose.dev.yml exec vpr grpcurl -plaintext localhost:50051 list`

Edge cases for automated edits
- Do not change workspace Cargo.toml members without verifying workspace resolution and builds for all crates.
- Avoid changing mount paths in `compose.dev.yml` — they are relied on by the dev container and CI.

If unsure, ask for clarification and provide a short plan: files to change, tests to add, and commands you will run to validate.

---
If you'd like I can expand any section (e.g., CI, proto build details, or example PR checklist).
