pub use ethers::types::{Address, Chain};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

const CONTRACTS_JSON: &str = include_str!("./addressbook.json");

static ADDRESSBOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    pub fn address(&self, chain: Chain) -> Option<Address> {
        self.addresses.get(&chain).cloned()
    }
}

pub fn contract_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    ADDRESSBOOK.get(&name.into()).and_then(|c| c.address(chain))
}
