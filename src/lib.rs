/*!
# Welcome to Saito

Saito is a **Tier 1 Blockchain Protocol** that incentivizes the provision of **high-throughput** network infrastructure. The network accomplishes with a consensus mechanism that pays the nodes in the peer-to-peer network for the collection and sharing of fees.

Saito-Rust is an implementation of Saito Consensus written in Rust for use by high-throughput routing nodes. It aims to offer the simplest and most scalable implementation of Saito Consensus.

If you need to get in touch with us, please reach out anytime.

See [readme for more details](https://github.com/SaitoTech/saito-rust/blob/main/README.md)

The Saito Team
dev@saito.tech

*/
pub mod block;
pub mod blockchain;
pub mod burnfee;
pub mod consensus;
pub mod crypto;
pub mod forktree;
pub mod golden_ticket;
pub mod keypair;
pub mod longest_chain_queue;
pub mod mempool;
pub mod slip;
pub mod storage;
pub mod time;
pub mod transaction;
pub mod types;
pub mod utxoset;

#[macro_use]
extern crate lazy_static;

// TODO move this to another file and include!()
// TODO put test_utilities behind a feature flag so it's not built into non-test builds
//   i.e. uncomment this line:
// [cfg(feature = "test-utilities")]
pub mod test_utilities;

/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// error handling crate or defining an error type as an `enum` of causes.
/// However, most time using a boxed `std::error::Error` is sufficient.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;
