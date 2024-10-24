use std::collections::BTreeMap;


pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u128>
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number (&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &String){
        let nonce = self.nonce.get(who).unwrap_or(&0) + 1;
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]

mod test {
    use super::Pallet;

    #[test]

    fn init_system() {
        let mut system = Pallet::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&"daniel".to_string()), None);

        system.inc_block_number();

        assert_eq!(system.block_number(), 1);
        
        system.inc_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get(&"daniel".to_string()).unwrap(), &1);
    }
}