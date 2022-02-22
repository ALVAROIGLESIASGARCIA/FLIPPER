#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};

#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        
        //Store some number
        my_number: u32,
        // Store some AccountId
        my_account: AccountId,
        // Store a mapping from AccountIds to a u32
        my_number_map: ink_storage::collections::HashMap<AccountId, u32>,
    }

    impl Incrementer {
        
        /// Constructor that initializes the `u32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self {
                my_number: init_value,
                my_account: Self::env().caller(),//Control de acceso para el possedor de la cuenta
                my_number_map: ink_storage::collections::HashMap::new(),
            }
        }
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self{
                my_number: 0,
                my_account: Default::default(),
                my_number_map: Default::default(),
            }
        }

        // Get the value for the calling AccountId
        #[ink(message)]
        pub fn get_my_number_map(&self) -> u32 {
            let caller = self.env().caller();
            self.my_number_or_zero(&caller)
        }
        /// Private function porque no lleva ni #[ink(message)] ni pub
        /// Returns the number for an AccountId or 0 if it is not set.
        fn my_number_or_zero(&self, of: &AccountId) -> u32 {
            let balance = self.my_number_map.get(of).unwrap_or(&0);
            *balance
        }
        /// A message that can be called on instantiated contracts.
        #[ink(message)]
        pub fn getNumber(&self) -> u32 {
            self.my_number
        }
        ///Atributo &mut para poder alterar el valor de storage 
        #[ink(message)]
        pub fn incrementar(&mut self, inc: u32) {
            self.my_number += inc;
        }

        #[ink(message)]
        pub fn getAccount(&self) -> AccountId {
            self.my_account
        }

        // Add a value to the existing value for the calling AccountId
        #[ink(message)]
        pub fn sumaNuevoNumber(&mut self, value: u32) {
            let caller = self.env().caller();
            let new_number = self.my_number_or_zero(&caller);
            self.my_number_map.insert(caller, new_number + value);
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(42);
            assert_eq!(incrementer.getNumber(), 42);
            incrementer.incrementar(5);
            assert_eq!(incrementer.getNumber(), 47);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.getNumber(), 11);
            assert_eq!(contract.get_my_number_map(), 0);
            contract.sumaNuevoNumber(5);
            assert_eq!(contract.get_my_number_map(), 5);
        }
    }
}
