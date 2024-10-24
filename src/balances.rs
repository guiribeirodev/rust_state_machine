use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<String, u128>
}

impl Pallet {
    pub fn new() -> Self {
        Pallet { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str>{
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}
#[cfg(test)]

mod tests {
    use std::u128;

    use super::Pallet;

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();
    
        assert_eq!(balances.balance(&"alice".to_string()), 0);
    
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
    
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]

    fn transfer_balance() {
        let mut balances = Pallet::new();

        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 10),
            Err("Insufficient balance")
        );

        balances.set_balance(&"daniel".to_string(), 10);
        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 3),
            Ok(())
        );
        assert_eq!(balances.balance(&"daniel".to_string()), 7);
        assert_eq!(balances.balance(&"vini".to_string()), 3);

        balances.set_balance(&"vini".to_string(), u128::MAX);
        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 3),
            Err("Overflow")
        );
    }
}

