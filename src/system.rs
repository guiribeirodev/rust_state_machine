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
}