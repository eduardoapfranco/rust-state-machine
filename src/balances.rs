use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}
#[macros::call]    
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient funds")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &T::AccountId, amount: T::Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u64;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();
        assert_eq!(balances.balance(&"Eduardo".to_string()), 0);
        balances.set_balance(&"Eduardo".to_string(), 20);
        assert_eq!(balances.balance(&"Eduardo".to_string()), 20);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();
        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 50);

        assert_eq!(
            balances.transfer("Alice".to_string(), "Bob".to_string(), 600),
            Err("Insufficient funds")
        );

        assert_eq!(
            balances.transfer("Alice".to_string(), "Bob".to_string(), 50),
            Ok(())
        );

        assert_eq!(balances.balance(&"Alice".to_string()), 50);
        assert_eq!(balances.balance(&"Bob".to_string()), 100);
    }
}
