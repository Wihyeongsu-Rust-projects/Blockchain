use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        // Success: create a claim
        poe.create_claim("Alice".to_string(), "my_document")
            .unwrap();
        assert_eq!(poe.get_claim(&"my_document"), Some(&"Alice".to_string()));

        // Error: revoke not owned claim
        let res = poe.revoke_claim("Bob".to_string(), "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        // Error: create existing claim
        let res = poe.create_claim("Bob".to_string(), "my_document");
        assert_eq!(res, Err("Claim already exists"));

        // Error: revoke non-existent claim
        let res = poe.revoke_claim("Alice".to_string(), "non_existent");
        assert_eq!(res, Err("Claim does not exist"));

        // Success: revoke owned claim
        let res = poe.revoke_claim("Alice".to_string(), "my_document");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"my_document"), None);
    }
}
