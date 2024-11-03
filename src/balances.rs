use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};


#[derive(Debug)]
pub struct Pallet <AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>
}

impl <AccountId, Balance> Pallet <AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: CheckedAdd + CheckedSub + Zero + Copy
{
    pub fn new() -> Self {
        Pallet { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str>{
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}
#[cfg(test)]

mod tests {
    use std::u128;

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<String, u128>::new();
    
        assert_eq!(balances.balance(&"alice".to_string()), 0);
    
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
    
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]

    fn transfer_balance() {
        let mut balances = super::Pallet::<String, u128>::new();

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

