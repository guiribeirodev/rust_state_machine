use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// O tipo que representa o conteúdo que pode ser reivindicado usando este pallet.
	/// Pode ser o conteúdo diretamente como bytes, ou melhor ainda, o hash desse conteúdo.
	/// Deixamos essa decisão para o desenvolvedor do runtime.
	type Content: Debug + Ord;
}

/// Este é o Módulo de Prova de Existência.
/// É um módulo simples que permite que contas reivindiquem a existência de alguns dados.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// Um simples mapa de armazenamento de conteúdo para o proprietário desse conteúdo.
	/// As contas podem fazer várias reivindicações diferentes, mas cada reivindicação só pode ter um proprietário.
	claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl <T: Config> Pallet<T> {
	/// Cria uma nova reivindicação em nome do `caller`.
	/// Esta função retornará um erro se alguém já tiver reivindicado esse conteúdo.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("This claim already exists");
		}
		
		self.claims.insert(claim, caller);

		Ok(())
	}

	/// Revoga uma reivindicação existente em algum conteúdo.
	/// Esta função só deve ter sucesso se o chamador for o proprietário de uma reivindicação existente.
	/// Retornará um erro se a reivindicação não existir ou se o chamador não for o proprietário.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.claims.get(&claim).ok_or("This claim does not exist")?;
		if owner != &caller {
			return Err("This claim does not belong to you");
		}

		self.claims.remove(&claim);

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	/// Cria uma nova instância do Módulo de Prova de Existência.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Obtém o proprietário (se houver) de uma reivindicação.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let alice = "Alice";
		let mut poe = super::Pallet::<TestConfig>::new();
		assert!(poe.get_claim(&"Hello").is_none());

		poe.create_claim(alice, "Hello");

		assert_eq!(poe.get_claim(&"Hello"), Some(&alice));
	}

	#[test]
	fn cant_claim_existing_claim() {
		let alice = "Alice";
		let bob = "Bob";
		let mut poe = super::Pallet::<TestConfig>::new();
		poe.create_claim(alice, "Hello");

		assert_eq!(poe.create_claim(bob, "Hello"), Err("This claim already exists"));
	}

	#[test]

	fn revoke_claim() {
		let alice = "Alice";
		let bob = "Bob";
		let mut poe = super::Pallet::<TestConfig>::new();
		poe.create_claim(alice, "Hello");

		assert_eq!(poe.revoke_claim(bob, "Hello"), Err("This claim does not belong to you"));
		assert_eq!(poe.revoke_claim(alice, "Hello"), Ok(()));
		assert!(poe.get_claim(&"Hello").is_none());

		assert!(poe.revoke_claim(bob, "No Claim Exists").is_err());
	}
}