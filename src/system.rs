use std::collections::BTreeMap;
use num::traits::{Zero, One};



#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>
}

impl <BlockNumber, AccountId, Nonce> Pallet <BlockNumber, AccountId, Nonce>
where
    BlockNumber: Zero + One + Copy,
    AccountId: Ord + Clone,
    Nonce: Zero + One + Copy
{
    pub fn new() -> Self {
        Pallet {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new()
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number (&mut self) {
        self.block_number = self.block_number + BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &AccountId){
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero()) + Nonce::one();
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]

mod test {
    use super::Pallet;

    #[test]

    fn init_system() {
        let mut system = Pallet::<u32, String, u128>::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&"daniel".to_string()), None);

        system.inc_block_number();

        assert_eq!(system.block_number(), 1);
        
        system.inc_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get(&"daniel".to_string()).unwrap(), &1);
    }

    #[test]

    fn increment_block_number(){
        let mut system = Pallet::<u32, String, u128>::new();

        assert_eq!(system.block_number(), 0);
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]

    fn inc_nonce() {
        let mut system = Pallet::<u32, String, u128>::new();

        system.inc_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get(&"daniel".to_string()).unwrap(), &1);
        system.inc_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get(&"daniel".to_string()).unwrap(), &2);
    }
}