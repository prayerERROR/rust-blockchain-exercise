# Rust Blockchain Exercise

## Overview

A simple blockchain implementation in Rust, including core concepts in blockchain.

Main features:

+ **Proof-of-work mining**: adjustable difficulty with nonce-based mining.
+ **Transaction process**: support regular transfers and coinbase transactions.
+ **Account state management**: support balance tracking.
+ **Merkle tree**: transaction verification with Merkle tree roots.
+ **Memory pool**: Transaction queue management.
+ **Block validation**: comprehensive block and transaction validation.
+ **Network layer**: Basic message passing for distributed nodes.

+ **Persistence**: Blockchain serialize and storage.

## Dependencies

+ **serde**: Serialization framework
+ **bincode**: Binary serialization
+ **chrono**: Date and time handling
+ **rust-crypto**: Cryptographic functions

## Limitations & Future Improvements

* Duplicated verification.
* No cryptographic signatures.
* No real P2P networking.

