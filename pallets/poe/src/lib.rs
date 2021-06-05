#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransferred(AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		// 创建存证
		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {			
			let sender = ensure_signed(origin)?;			
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);			
			Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));			
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));			
			Ok(())
		}

		//撤销存证
		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			let (owner, _) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			Ok(())
		}

		// 转移存证，可调用函数需要多一个参数
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, account: T::AccountId) -> dispatch::DispatchResult {
			// 确保交易被某用户签名，并获取发送者accountId
			let sender = ensure_signed(origin)?;
			// 确保存证存在
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			// 获取存证owner
			let (owner, _) = Proofs::<T>::get(&claim);
			// 确保用户一致
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			// 存储单元转移
			Proofs::<T>::insert(&claim, (account.clone(), frame_system::Module::<T>::block_number()));
			// 触发事件
			Self::deposit_event(RawEvent::ClaimTransferred(account, claim));
			
			Ok(())
		}
	}
}
