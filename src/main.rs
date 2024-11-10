mod balances;
mod proof_of_existence;
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
    pub type Block = crate::support::Block<Header, Extrinsic>;

    pub type Content = String;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                format!(
                    "Error in block {}: extrinsic {}: {}",
                    block.header.block_number, i, e
                )
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
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    use balances::Call::*;
    use proof_of_existence::Call::*;

    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(create_claim {
                    claim: "Hello Blockchain!".to_string(),
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(create_claim {
                    claim: "Document Car Chevrolet".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(create_claim {
                    claim: "Document Car Chevrolet".to_string(),
                }),
            },
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::ProofOfExistence(create_claim {
                    claim: "Hello Blockchain!".to_string(),
                }),
            },
        ],
    };

    runtime.execute_block(block_2).expect("invalid block");

    println!("{:#?}", runtime);
}
