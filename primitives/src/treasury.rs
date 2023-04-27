use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct Proposal<AccountId, Balance, Hash> {
	/// The account proposing it.
	pub proposer: AccountId,
	/// The (total) amount that should be paid if the proposal is accepted.
	pub value: Balance,
	/// The account to whom the payment should be made if the proposal is accepted.
	pub beneficiary: AccountId,
	/// The amount held on deposit (reserved) for making this proposal.
	pub bond: Balance,
	/// Hash of the meme_reference struct
	pub meme_reference_hash: Hash,
}
