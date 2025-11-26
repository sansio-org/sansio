//! Async local executor
//!
//! This module provides a local executor abstraction that can be backed by either
//! smol or tokio, depending on the enabled feature flags.
//!
//! ## Features
//! - `runtime-smol` (default): Use smol's LocalExecutor
//! - `runtime-tokio`: Use tokio's LocalSet
//!
//! ## Examples
//!
//! ### Basic Usage
//! ```rust,no_run
//! use sansio::LocalExecutorBuilder;
//!
//! LocalExecutorBuilder::default()
//!     .run(async {
//!         println!("Hello from local executor!");
//!     });
//! ```
//!
//! ### With CPU Pinning
//! ```rust,no_run
//! use sansio::LocalExecutorBuilder;
//! use core_affinity::CoreId;
//!
//! LocalExecutorBuilder::new()
//!     .name("my-executor")
//!     .core_id(CoreId { id: 0 })
//!     .run(async {
//!         println!("Running on CPU core 0!");
//!     });
//! ```
//!
//! ### Spawning Local Tasks
//! ```rust,no_run
//! use sansio::{LocalExecutorBuilder, spawn_local};
//!
//! LocalExecutorBuilder::default().run(async {
//!     let task = spawn_local(async {
//!         println!("Hello from spawned task!");
//!         42
//!     });
//!
//!     let result = task.await;
//!     println!("Task returned: {}", result);
//! });
//! ```

// =============================================================================
// Smol-based implementation
// =============================================================================

#[cfg(feature = "runtime-smol")]
mod smol;

#[cfg(feature = "runtime-smol")]
pub use smol::*;

// =============================================================================
// Tokio-based implementation
// =============================================================================

#[cfg(feature = "runtime-tokio")]
mod tokio;

#[cfg(feature = "runtime-tokio")]
pub use tokio::*;

// =============================================================================
// Compile-time guards
// =============================================================================

// Compile error if neither or both features are enabled
#[cfg(not(any(feature = "runtime-smol", feature = "runtime-tokio")))]
compile_error!("Either 'runtime-smol' or 'runtime-tokio' feature must be enabled");

#[cfg(all(feature = "runtime-smol", feature = "runtime-tokio"))]
compile_error!("Only one runtime feature can be enabled at a time: 'runtime-smol' or 'runtime-tokio'");
