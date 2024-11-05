mod balances;
mod system;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u64;
    pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self>
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new()
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();
    assert!(runtime.system.block_number() == 1);

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e|println!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e|println!("{}", e));

    println!("{:#?}", runtime);
}
