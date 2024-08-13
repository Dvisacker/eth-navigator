pub use ethers::types::{Address, Chain};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

const CONTRACTS_JSON: &str = include_str!("./addressbook.json");

static ADDRESSBOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    /// Returns the address of the contract on the specified chain. If the contract's address is
    /// not found in the addressbook, the getter returns None.
    pub fn address(&self, chain: Chain) -> Option<Address> {
        self.addresses.get(&chain).cloned()
    }
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of
/// [ethers-addressbook](https://github.com/gakonst/ethers-rs/tree/master/ethers-addressbook) we return None.
pub fn contract_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    ADDRESSBOOK.get(&name.into()).and_then(|c| c.address(chain))
}

// tests
