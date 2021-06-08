#![cfg_attr(not(feature = "std"), no_std)]

//! Simple Token Transfer
//! 1. set total supply
//! 2. establish ownership upon configuration of circulating tokens
//! 3. coordinate token transfers with the runtime functions
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;



pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}



decl_storage! {
	trait Store for Module<T: Config> as Quanstake {
		pub QuanStakes get(fn get_quan): map hasher(blake2_128_concat) u64 => T::AccountId;
		pub SuperManager get(fn is_manager): map hasher(blake2_128_concat) T::AccountId => T::AccountId;
		pub Init get(fn is_init): bool;
		pub QuanCount get(fn is_index): u64;


	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		BlockNumber = <T as frame_system::Config>::BlockNumber,
	{
		/// Token was initialized by user
		Initialized(AccountId,BlockNumber),
		/// Tokens successfully transferred between users
		AddQuanNodeed(AccountId, AccountId,BlockNumber), // (from, to, value)
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
		NotManager,
		AlreadyAdd,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

	
		/// set super manager
		#[weight = 10_000]
		fn init(_origin,manager:T::AccountId) -> DispatchResult {
			let sender = ensure_signed(_origin)?;
			ensure!(!Self::is_init(), <Error<T>>::AlreadyInitialized);
			
			QuanCount::put(1u64);
			<SuperManager<T>>::insert(&sender,&manager);
			
			Init::put(true);
			// Emit an event.
			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::Initialized(manager, now));
			Ok(())
		}

		/// Transfer tokens from one account to another
		#[weight = 10_000]
		fn add_quan_node(_origin, quan_node: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::is_manager(&sender) == sender,<Error<T>>::NotManager);
			//ensure!(Self::get_quan(&quan_node) != quan_node ,<Error<T>>::AlreadyAdd);

			let index = Self::is_index()+ 1 ;
			
			<QuanStakes<T>>::insert(index,&quan_node);

			QuanCount::put(index);
			
			let now = <frame_system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::AddQuanNodeed(sender, quan_node,now));
			Ok(())
		}
		
	}
}