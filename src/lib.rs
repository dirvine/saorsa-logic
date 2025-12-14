// Copyright 2024 Saorsa Labs Limited
//
// Licensed under the Apache License, Version 2.0 or MIT license, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

//! # saorsa-logic
//!
//! Pure logic crate for the Saorsa network, designed for zkVM compatibility.
//!
//! This crate provides the core verification logic that can be executed inside
//! zero-knowledge virtual machines (zkVMs) like SP1 or RISC Zero. All code is
//! `no_std` compatible and deterministic.
//!
//! ## Design Philosophy
//!
//! The Saorsa network uses "Entangled Attestation" to ensure nodes run authorized
//! software. This crate extracts the pure verification logic so that:
//!
//! 1. **zkVM Proofs**: Nodes can prove they computed their EntangledId correctly
//! 2. **Data Verification**: Storage operations can be proven correct
//! 3. **Determinism**: All operations are deterministic and reproducible
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        saorsa-node                              │
//! │  (Thin wrapper: CLI, config, auto-upgrade)                      │
//! └────────────────────────────┬────────────────────────────────────┘
//!                              │
//! ┌────────────────────────────▼────────────────────────────────────┐
//! │                        saorsa-core                              │
//! │  (Networking, DHT, trust, storage coordination)                 │
//! └────────────────────────────┬────────────────────────────────────┘
//!                              │
//! ┌────────────────────────────▼────────────────────────────────────┐
//! │                       saorsa-logic                              │
//! │  (Pure verification logic - THIS CRATE)                         │
//! │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐             │
//! │  │ attestation  │ │    data      │ │   merkle     │             │
//! │  │ (EntangledId)│ │ (hash, sig)  │ │  (proofs)    │             │
//! │  └──────────────┘ └──────────────┘ └──────────────┘             │
//! └─────────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//!                    ┌─────────────────┐
//!                    │  zkVM (SP1)     │
//!                    │  Proves logic   │
//!                    │  is correct     │
//!                    └─────────────────┘
//! ```
//!
//! ## Usage
//!
//! ### Native (with std)
//!
//! ```rust
//! use saorsa_logic::attestation::{derive_entangled_id, verify_entangled_id};
//!
//! // Derive an entangled identity
//! let public_key = [0u8; 1952]; // ML-DSA-65 public key
//! let binary_hash = [1u8; 32];   // BLAKE3 hash of binary
//! let nonce = 12345u64;
//!
//! let entangled_id = derive_entangled_id(&public_key, &binary_hash, nonce);
//!
//! // Verify the identity
//! assert!(verify_entangled_id(&entangled_id, &public_key, &binary_hash, nonce));
//! ```
//!
//! ### In zkVM (SP1)
//!
//! ```rust,ignore
//! // In SP1 guest program
//! use saorsa_logic::attestation::derive_entangled_id;
//!
//! // Read inputs from prover
//! let public_key: [u8; 1952] = sp1_zkvm::io::read();
//! let binary_hash: [u8; 32] = sp1_zkvm::io::read();
//! let nonce: u64 = sp1_zkvm::io::read();
//!
//! // Compute EntangledId (this computation is proven)
//! let entangled_id = derive_entangled_id(&public_key, &binary_hash, nonce);
//!
//! // Commit result to public outputs
//! sp1_zkvm::io::commit(&entangled_id);
//! ```
//!
//! ## Features
//!
//! - `std`: Enable standard library support (for native execution)
//! - `alloc`: Enable heap allocation
//! - `zkvm`: Generic zkVM optimizations
//! - `sp1`: SP1-specific optimizations
//! - `risc0`: RISC Zero-specific optimizations
//!
//! ## no_std Compatibility
//!
//! This crate is `no_std` by default. To use with std:
//!
//! ```toml
//! saorsa-logic = { version = "0.1", features = ["std"] }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// Allow in no_std context where Result is commonly used
#![allow(clippy::missing_errors_doc)]
// Allow technical terms without backticks in docs
#![allow(clippy::doc_markdown)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod attestation;
pub mod data;
pub mod error;
pub mod merkle;

// Re-exports for convenience
pub use attestation::{derive_entangled_id, verify_entangled_id, EntangledIdComponents};
pub use data::{compute_content_hash, verify_content_hash};
pub use error::{LogicError, LogicResult};
