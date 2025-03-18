use num::{One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Copy + AddAssign;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = self.nonce(who);

        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
        system.inc_block_number();
        assert_eq!(system.block_number(), 2);
    }

    #[test]
    fn inc_nonce() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        system.inc_nonce(&"Alice".to_string());
        assert_eq!(system.nonce(&"Alice".to_string()), 1);
    }
}
