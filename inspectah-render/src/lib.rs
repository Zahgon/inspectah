//! Rendering of bootc build artifacts from an inspection snapshot.
//!
//! Each renderer is a pure function of the snapshot: it reads no host state
//! and performs no I/O beyond writing the artifact it was asked for.
pub mod render;
