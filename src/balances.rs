use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>
}

impl Pallet {
    pub fn new() -> Self {
        Pallet { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: String) -> u128 {
        *self.balances.get(&who).unwrap_or(&0)
    }
}

#[test]
fn init_balances() {
    let mut balances = Pallet::new();

    assert_eq!(balances.balance("alice".to_string()), 0);

    balances.set_balance("alice".to_string(), 100);
    assert_eq!(balances.balance("alice".to_string()), 100);

    assert_eq!(balances.balance("bob".to_string()), 0);
}