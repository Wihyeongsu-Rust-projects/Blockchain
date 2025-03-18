use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce(who);

        self.nonce
            .insert(who.clone(), nonce.checked_add(1).unwrap());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
        system.inc_block_number();
        assert_eq!(system.block_number(), 2);
    }

    #[test]
    fn inc_nonce() {
        let mut system = super::Pallet::new();

        system.inc_nonce(&"Alice".to_string());
        assert_eq!(system.nonce(&"Alice".to_string()), 1);
    }
}
