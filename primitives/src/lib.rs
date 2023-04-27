#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature,
};

/// Alias to type for a signature for a transaction. This allows one of several
/// kinds of underlying crypto to be used, so isn't a fixed size when encoded.
pub type Signature = MultiSignature;

/// Alias to the public key used for the chain. Like the signature, this
/// also isn't a fixed size when encoded, as different cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Alias to the opaque account ID type for the chain, actually a `AccountId32`.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

mod meme_reference;
pub mod treasury;

pub use meme_reference::MemeReference;
pub use treasury::Proposal;
