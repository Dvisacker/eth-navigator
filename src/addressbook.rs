pub use ethers::types::{Address, Chain};
use once_cell::sync::Lazy;
use std::collections::HashMap;

const CONTRACTS_JSON: &str = include_str!("./addressbook.json");

static ADDRESSBOOK: Lazy<HashMap<String, HashMap<String, Address>>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

pub fn contract_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    ADDRESSBOOK
        .get(&name.into())
        .and_then(|addresses| addresses.get(&chain.to_string().to_lowercase()).cloned())
}
