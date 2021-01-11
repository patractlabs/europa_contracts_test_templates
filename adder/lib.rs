#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod add {
    use accumulator::Accumulator;
    use ink_env::call::FromAccountId;

    /// Defines the storage of your contract.
        /// Add new fields to the below struct in order
        /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Add {
        /// Stores a single `bool` value on the storage.
        accumulator: Accumulator,
    }

    impl Add {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(addr: AccountId) -> Self {
            let accumulator: Accumulator = FromAccountId::from_account_id(addr);
            // let acc = Accumulator{ account_id: addr};
            Self { accumulator }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.accumulator.get()
        }

        #[ink(message)]
        pub fn add(&mut self, by: i32) {
            self.accumulator.inc(by)
        }

        #[ink(message)]
        pub fn adder_panic(&mut self) {
            self.accumulator.acc_panic();
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let add = Add::default();
            assert_eq!(add.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut add = Add::new(1);
            assert_eq!(add.get(), 1);
            add.add(1);
            assert_eq!(add.get(), 2);
        }
    }
}
