use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: CheckedAdd+ CheckedSub + Zero + Copy;
}

pub enum Call<T: Config> {
    Transfer {to: T::AccountId, value: T::Balance}
}

#[derive(Debug)]
pub struct Pallet <T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>
}

impl <T:Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    // Despacha uma chamada em nome de um chamador. Aumenta o nonce do chamador.
    //
    // Dispatch nos permite identificar qual chamada de módulo subjacente queremos executar.
    // Observe que extraímos o `chamador` do extrínseco e usamos essa informação
    // para determinar em nome de quem estamos executando a chamada.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, value } => {
                self.transfer(caller, to, value)
            }
        }
    }
}

impl <T:Config> Pallet <T> {
    pub fn new() -> Self {
        Pallet { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
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

    struct TestConfig;

    impl  super::Config for TestConfig{
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }



    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();
    
        assert_eq!(balances.balance(&"alice".to_string()), 0);
    
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
    
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]

    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();

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

