use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::types::Address;
use openzeppelin_rs::ERC20;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub chain_id: u64,
    pub symbol: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Whitelist {
    wallet_addresses: HashMap<String, WalletInfo>,
    token_addresses: HashMap<String, TokenInfo>,
}

impl Whitelist {
    pub fn new() -> Self {
        Whitelist {
            wallet_addresses: HashMap::new(),
            token_addresses: HashMap::new(),
        }
    }

    pub fn add_wallet_address(&mut self, address: String, name: Option<String>) {
        self.wallet_addresses
            .insert(address.clone(), WalletInfo { address, name });
    }

    pub fn remove_wallet_address(&mut self, address: &str) {
        self.wallet_addresses.remove(address);
    }

    pub async fn add_token_address(
        &mut self,
        address: String,
        chain_id: u64,
        name: Option<String>,
        provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let address = Address::from_str(&address)?;
        let token = ERC20::new(address, provider);
        let symbol = token.symbol().call().await?;
        self.token_addresses.insert(
            format!("{}:{}", address, chain_id),
            TokenInfo {
                address: address.to_string(),
                chain_id,
                symbol,
                name,
            },
        );
        Ok(())
    }

    pub fn remove_token_address(&mut self, address: &str, chain_id: u64) {
        self.token_addresses
            .remove(&format!("{}:{}", address, chain_id));
    }

    pub fn is_wallet_whitelisted(&self, address: &str) -> bool {
        self.wallet_addresses.contains_key(address)
    }

    pub fn is_token_whitelisted(&self, address: &str, chain_id: u64) -> bool {
        self.token_addresses
            .contains_key(&format!("{}:{}", address, chain_id))
    }

    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let whitelist: Whitelist = serde_json::from_str(&file_content)?;
        Ok(whitelist)
    }

    pub fn get_wallet_addresses(&self) -> &HashMap<String, WalletInfo> {
        &self.wallet_addresses
    }

    pub fn get_token_addresses(&self) -> &HashMap<String, TokenInfo> {
        &self.token_addresses
    }

    pub fn get_wallet_by_name(&self, name: &str) -> Option<&WalletInfo> {
        self.wallet_addresses
            .values()
            .find(|info| info.name.as_deref() == Some(name))
    }
}

impl Default for Whitelist {
    fn default() -> Self {
        Self::new()
    }
}
