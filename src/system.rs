use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type BlockNumber: Zero + One + AddAssign + Copy;
    type AccountId: Ord + Clone;
    type Nonce: Zero + Copy + One + AddAssign;
}


#[derive(Debug)]
pub struct Pallet<T:Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T:Config> Pallet<T>
{
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, account: &T::AccountId) {
        let nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {

    struct TestConfig;
    impl  super::Config for TestConfig{
        type BlockNumber = u32;
        type AccountId = u32;
        type Nonce = u32;
    }
    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();
        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&1), None);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&1);
        assert_eq!(system.nonce.get(&1).unwrap(), &1);
    }

    #[test]
    fn init_nonce() {
        let mut system = super::Pallet::<TestConfig>::new();

        system.inc_nonce(&1);
        assert_eq!(system.nonce.get(&1).unwrap(), &1);
        system.inc_nonce(&1);
        assert_eq!(system.nonce.get(&1).unwrap(), &2);
    }

    #[test]
    fn init_inc_block_numb(){
        let mut system = super::Pallet::<TestConfig>::new();
        system.inc_block_number();
    }
}
