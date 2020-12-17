#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode},
    debug, decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap,
};
use frame_system::ensure_signed;
use sp_runtime::print;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct PersonData {
    age: u32,
    sex: u32,
    bmi: u32,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        /// The storage item for our proofs.
        /// It maps a proof to the user who made the claim and when they made it.
        // Persons get(fn persons): map hasher(blake2_128_concat) T::AccountId => PersonData;
        Persons get(fn persons): map hasher(blake2_128_concat) T::AccountId => (u32,u32,u32);

        BloodData get(fn blood_data): map hasher(blake2_128_concat) T::AccountId => (u32,u32,u32,u32,u32,u32,u32);

        Thing1 get(fn thing1): u32;
        Thing2 get(fn thing2): u32;
    }
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        /// Event emitted when a person has been created. [who]
        PersonCreated(AccountId),

        /// Event emitted when a person has been updated. [who]
        PersonUpdated(AccountId),

        /// Event emitted when a person has been inserted blood data. [who]
        BloodDataInserted(AccountId),

        /// Event emitted when a person has been requested diabet prediction. [who]
        PredictDiabet(AccountId),

        /// Set new value. [key] [value]
        ValueSet(u32, u32),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        NotEnoughDataFields
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

        /// Allow a user to claim ownership of an unclaimed proof.
        #[weight = 10_000]
        fn create_person(origin, age: u32, sex: u32, bmi: u32) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            debug::info!("Create person request from {:?}", sender);

            /*let body = PersonData {
                age,
                sex,
                bmi,
            };*/

             // Store the proof with the sender and block number.
            Persons::<T>::insert(&sender,(age,sex,bmi));

            // Emit an event that the person was created.
            Self::deposit_event(RawEvent::PersonCreated(sender));
        }

        /// Allow a user to claim ownership of an unclaimed proof.
        #[weight = 10_000]
        fn insert_blood(origin, bp: u32, sr1: u32, sr2: u32, sr3: u32, sr4: u32, sr5: u32, sr6: u32) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            debug::info!("Insert blood data request from {:?}", sender);

             // Store the proof with the sender and block number.
            BloodData::<T>::insert(&sender,(bp,sr1,sr2,sr3,sr4,sr5,sr6));

            // Emit an event that the person was created.
            Self::deposit_event(RawEvent::BloodDataInserted(sender));
        }

        /// Allow a user to claim ownership of an unclaimed proof.
        #[weight = 20_000]
        fn prdict_diabet(origin) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            debug::info!("Predict diabet request from {:?}", sender);


            // let val: f64 = 0.1 * thread_rng().sample::<f64,_>(StandardNormal);
            let val: f64 = 0.088;


            debug::info!("{}", val);

        }

        /// Sets the first simple storage value
        #[weight = 10_000]
        pub fn set_thing_1(origin, val: u32) {
            let _ = ensure_signed(origin)?;

            Thing1::put(val);

            Self::deposit_event(RawEvent::ValueSet(1, val));
        }

        /// Sets the second stored value
        #[weight = 10_000]
        pub fn set_thing_2(origin, val: u32) {
            let _ = ensure_signed(origin)?;

            Thing2::put(val);

            Self::deposit_event(RawEvent::ValueSet(2, val));
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn get_sum() -> u32 {
        Thing1::get() + Thing2::get()
    }
}
