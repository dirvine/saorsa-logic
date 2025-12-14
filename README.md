# saorsa-logic

Pure verification logic for the Saorsa network, designed for zkVM compatibility.

[![Crates.io](https://img.shields.io/crates/v/saorsa-logic.svg)](https://crates.io/crates/saorsa-logic)
[![Documentation](https://docs.rs/saorsa-logic/badge.svg)](https://docs.rs/saorsa-logic)

## Overview

`saorsa-logic` extracts the core verification logic from the Saorsa network into a `no_std` compatible crate. This enables:

1. **zkVM Proofs**: Run verification logic inside SP1/RISC Zero to generate proofs
2. **Deterministic Execution**: All operations are pure and reproducible
3. **Minimal Dependencies**: Only BLAKE3 and serde for maximum portability

## Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                        saorsa-node                              │
│  (CLI, config, auto-upgrade)                                    │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                        saorsa-core                              │
│  (Networking, DHT, trust, storage)                              │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       saorsa-logic                              │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐             │
│  │ attestation  │ │    data      │ │   merkle     │             │
│  └──────────────┘ └──────────────┘ └──────────────┘             │
└─────────────────────────────────────────────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │   zkVM (SP1)    │
                    │  Proves logic   │
                    └─────────────────┘
```

## Features

### Attestation Module

Implements Entangled Attestation - binding node identity to software:

```rust
use saorsa_logic::attestation::{derive_entangled_id, verify_entangled_id};

let public_key = [0u8; 1952]; // ML-DSA-65 public key
let binary_hash = [1u8; 32];   // BLAKE3 of binary
let nonce = 12345u64;

// Derive EntangledId
let id = derive_entangled_id(&public_key, &binary_hash, nonce);

// Verify
assert!(verify_entangled_id(&id, &public_key, &binary_hash, nonce));
```

### Data Module

Content-addressing and verification:

```rust
use saorsa_logic::data::{compute_content_hash, verify_content_hash};

let data = b"Hello, Saorsa!";
let hash = compute_content_hash(data);

assert!(verify_content_hash(data, &hash).is_ok());
```

### Merkle Module

Merkle tree construction and proof verification:

```rust
use saorsa_logic::merkle::{build_tree_root, generate_proof, hash_leaf};

let leaves: Vec<[u8; 32]> = (0..4).map(|i| hash_leaf(&[i])).collect();
let root = build_tree_root(&leaves);

let proof = generate_proof(&leaves, 0).unwrap();
assert!(proof.verify(&leaves[0], &root).is_ok());
```

## zkVM Usage

### SP1 Guest Program

```rust,ignore
// In your SP1 guest program
use saorsa_logic::attestation::derive_entangled_id;

fn main() {
    // Read inputs
    let public_key: Vec<u8> = sp1_zkvm::io::read();
    let binary_hash: [u8; 32] = sp1_zkvm::io::read();
    let nonce: u64 = sp1_zkvm::io::read();

    // Compute (this is what gets proven)
    let entangled_id = derive_entangled_id(&public_key, &binary_hash, nonce);

    // Commit to public outputs
    sp1_zkvm::io::commit(&entangled_id);
    sp1_zkvm::io::commit(&binary_hash);
}
```

## Feature Flags

- `std` - Enable standard library (for native execution)
- `alloc` - Enable heap allocation
- `zkvm` - Generic zkVM optimizations
- `sp1` - SP1-specific optimizations
- `risc0` - RISC Zero-specific optimizations
- `test-utils` - Testing utilities

## no_std Compatibility

This crate is `no_std` by default:

```toml
# no_std (zkVM)
saorsa-logic = "0.1"

# With std (native)
saorsa-logic = { version = "0.1", features = ["std"] }
```

## Security Properties

- **Deterministic**: Same inputs always produce same outputs
- **Constant-time**: Comparisons use constant-time operations
- **Domain separated**: Hash prefixes prevent cross-type collisions
- **Collision resistant**: BLAKE3 provides 256-bit security

## License

MIT OR Apache-2.0
