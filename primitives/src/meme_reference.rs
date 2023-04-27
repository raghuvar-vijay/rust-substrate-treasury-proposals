use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;


#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Hash))]
pub struct MemeReference {
	pub owner_name: Vec<u8>,
	pub meme_name: Vec<u8>,
}
