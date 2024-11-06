use types::AccountId;

mod balances;
mod support;
mod system;

use crate::support::Dispatch;


mod types {
    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u64;
    pub type Nonce = u32;

    
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block =  crate::support::Block<Header, Extrinsic>;
}


pub enum RuntimeCall {}


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
    type Balance = types::Balance;
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new()
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic {caller, call}) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                format!("Error in block {}: extrinsic {}: {}", block.header.block_number, i, e)
            });
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    // Despacha uma chamada em nome de um chamador. Aumenta o nonce do chamador.
    //
    // Dispatch nos permite identificar qual chamada de módulo subjacente queremos executar.
    // Observe que extraímos o `chamador` do extrínseco e usamos essa informação
    // para determinar em nome de quem estamos executando a chamada.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        unimplemented!();
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
