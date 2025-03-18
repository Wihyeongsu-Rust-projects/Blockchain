use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    struct TestConfig;
    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balances.balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        balances
            .transfer(&"Alice".to_string(), &"Bob".to_string(), 50)
            .unwrap();
        assert_eq!(balances.balance(&"Alice".to_string()), 50);
        assert_eq!(balances.balance(&"Bob".to_string()), 100);
    }

    #[test]
    fn transfer_balance_insufficient() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        let result = balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 200);
        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 50);
    }

    #[test]
    fn transfer_balance_overflow() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), u128::MAX);

        let result = balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 1);
        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), u128::MAX);
    }
}
