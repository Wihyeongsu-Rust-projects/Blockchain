use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        balances
            .transfer(&"Alice".to_string(), &"Bob".to_string(), 50)
            .unwrap();
        assert_eq!(balances.balance(&"Alice".to_string()), 50);
        assert_eq!(balances.balance(&"Bob".to_string()), 100);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let mut balances = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        let result = balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 200);
        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), 50);
    }

    #[test]
    fn transfer_balance_overflow() {
        let mut balances = super::Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), u128::MAX);

        let result = balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 1);
        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&"Alice".to_string()), 100);
        assert_eq!(balances.balance(&"Bob".to_string()), u128::MAX);
    }
}
