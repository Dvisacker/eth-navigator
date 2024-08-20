use crate::addressbook;
use crate::whitelist::Whitelist;
use ethers::types::{Address, Chain};
use std::str::FromStr;
use std::sync::Arc;

pub struct Resolver {
    whitelist: Arc<Whitelist>,
}

impl Resolver {
    pub fn new(whitelist: Arc<Whitelist>) -> Self {
        Resolver { whitelist }
    }

    pub fn resolve(
        &self,
        input: &str,
        chain: Chain,
    ) -> Result<Address, Box<dyn std::error::Error>> {
        // Check if the input is a valid Ethereum address
        if let Ok(address) = Address::from_str(input) {
            return Ok(address);
        }

        // Check if the input is a whitelisted wallet name
        if let Some(wallet_info) = self.whitelist.get_wallet_by_name(input) {
            return Ok(Address::from_str(&wallet_info.address)?);
        }

        // Check if the input is a token name in the addressbook
        if let Some(address) = addressbook::contract_address(input, chain) {
            return Ok(address);
        }

        Err(format!("Could not resolve address for: {}", input).into())
    }
}
