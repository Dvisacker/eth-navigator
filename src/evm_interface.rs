use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::{Address, Filter, H256, U64};
use openzeppelin_rs::ERC20;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use crate::bindings::weth::WETH;
use crate::config::{get_chain_config, get_chain_from_string, ChainConfig};
use crate::{addressbook, utils};

pub struct EVMInterface {
    config: Arc<ChainConfig>,
    explorer_client: Arc<Client>,
    network: String,
}

impl EVMInterface {
    pub async fn new(network: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chain = get_chain_from_string(network)
            .ok_or_else(|| format!("Unsupported network: {}", network))?;
        let config = get_chain_config(chain).await;
        let explorer_api_key = config.explorer_api_key.clone();
        let explorer_client = Client::new(chain, explorer_api_key)?;
        Ok(Self {
            config: Arc::new(config),
            network: network.to_string(),
            explorer_client: Arc::new(explorer_client),
        })
    }

    pub async fn get_block_number(&self) -> Result<(), Box<dyn std::error::Error>> {
        let block_number: U64 = self.config.http.get_block_number().await?;
        println!("Current block number on {}: {block_number}", self.network);
        Ok(())
    }

    pub async fn subscribe_blocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.config.ws.subscribe_blocks().await?;
        println!("Subscribing to new blocks on {}...", self.network);
        while let Some(block) = stream.next().await {
            println!("New block: {:?}", block.number.unwrap());
        }
        Ok(())
    }

    pub async fn subscribe_pending_transactions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.config.ws.subscribe_pending_txs().await?;
        println!("Subscribing to pending transactions on {}...", self.network);
        while let Some(tx_hash) = stream.next().await {
            println!("Pending transaction: {:?}", tx_hash);
        }
        Ok(())
    }

    pub async fn get_gas_price(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gas_price = self.config.http.get_gas_price().await?;
        println!("Current gas price on {}: {} wei", self.network, gas_price);
        println!(
            "Current gas price on {}: {} gwei",
            self.network,
            gas_price.as_u64() as f64 / 1_000_000_000.0
        );
        Ok(())
    }

    pub async fn get_balance(&self, address: String) -> Result<(), Box<dyn std::error::Error>> {
        let address = Address::from_str(&address)?;
        let balance = self.config.http.get_balance(address, None).await?;
        println!(
            "Balance of {} on {}: {} wei",
            address, self.network, balance
        );
        println!(
            "Balance in ETH: {} ETH",
            ethers::utils::format_ether(balance)
        );
        Ok(())
    }

    pub async fn get_nonce(&self, address: String) -> Result<(), Box<dyn std::error::Error>> {
        let address = Address::from_str(&address)?;
        let nonce = self
            .config
            .http
            .get_transaction_count(address, None)
            .await?;

        println!(
            "Nonce for address {} on {}: {}",
            address, self.network, nonce
        );
        Ok(())
    }

    pub async fn get_block_details(
        &self,
        block_number: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let block = self.config.http.get_block(block_number).await?;

        match block {
            Some(block) => {
                println!(
                    "Block details for block {} on {}:",
                    block_number, self.network
                );
                utils::print_block_details(&block);
            }
            None => println!("Block {} not found on {}", block_number, self.network),
        }

        Ok(())
    }

    pub async fn subscribe_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.config.ws.subscribe_logs(&Filter::new()).await?;
        println!("Subscribing to logs on {}...", self.network);
        while let Some(log) = stream.next().await {
            println!("New log: {:?}", log);
        }

        Ok(())
    }

    pub async fn get_tx_details(&self, tx_hash: String) -> Result<(), Box<dyn std::error::Error>> {
        let tx_hash = H256::from_str(&tx_hash)?;
        let tx = self.config.http.get_transaction(tx_hash).await?;

        match tx {
            Some(tx) => {
                println!("Transaction details for {:?} on {}:", tx_hash, self.network);
                utils::print_tx_details(&tx);
            }
            None => println!("Transaction {:?} not found on {}", tx_hash, self.network),
        }

        Ok(())
    }

    pub async fn generate_contract_bindings(
        &self,
        contract_address: String,
        contract_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Generating contract bindings for {} on {}",
            contract_address, self.network
        );

        println!(
            "Fetching ABI for contract {} on {}...",
            contract_address, self.network
        );

        let abi: Abi = self
            .explorer_client
            .contract_abi(contract_address.parse()?)
            .await?;
        let abi_str = serde_json::to_string(&abi)?;
        let bindings_dir = Path::new("./src/bindings");
        fs::create_dir_all(bindings_dir)?;

        let output_file = bindings_dir.join(format!("{}.rs", contract_name.to_lowercase()));
        let bindings = Abigen::new(&contract_name, abi_str)?.generate()?;

        fs::write(&output_file, bindings.to_string())?;
        println!("Bindings generated and saved to {:?}", output_file);

        Ok(())
    }

    pub async fn generate_source_code(
        &self,
        contract_address: String,
        contract_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Downloading contract and generating bindings for {} on {}",
            contract_address, self.network
        );

        println!(
            "Downloading ABI and source code for contract {}...",
            contract_address
        );
        let source_code = self
            .explorer_client
            .contract_source_code(contract_address.parse()?)
            .await?;

        let bindings_dir = Path::new("./src/bindings");
        fs::create_dir_all(bindings_dir)?;

        for (index, contract) in source_code.items.iter().enumerate() {
            // Save ABI
            let abi_file = bindings_dir.join(format!("{}_abi.json", contract_name.to_lowercase()));
            println!("Saving ABI to {:?}", abi_file);
            fs::write(&abi_file, &contract.abi)?;

            // Save source code
            let source_file =
                bindings_dir.join(format!("{}_source.sol", contract_name.to_lowercase()));
            println!("Saving source code to {:?}", source_file);
            fs::write(&source_file, contract.source_code())?;

            // Generate bindings
            println!("Generating bindings for {}...", contract.contract_name);
            let output_file = bindings_dir.join(format!("{}.rs", contract_name.to_lowercase()));
            let bindings =
                Abigen::new(&contract.contract_name, contract.abi.clone())?.generate()?;
            println!("Saving bindings to {:?}", output_file);
            fs::write(&output_file, bindings.to_string())?;

            println!(
                "Contract {} processed successfully!",
                contract.contract_name
            );
        }

        println!("All contracts downloaded and bindings generated successfully!");
        Ok(())
    }

    pub async fn get_erc_20_balances(
        &self,
        wallet_address: String,
        token_address: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token_address = Address::from_str(&token_address)?;
        let wallet_address = Address::from_str(&wallet_address)?;
        let token = ERC20::new(token_address, self.config.http.clone());
        let balance = token.balance_of(wallet_address).await?;

        println!(
            "Balance of {} on {}: {}",
            wallet_address, self.network, balance
        );
        Ok(())
    }

    pub async fn wrap_eth(&self, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        let weth_address =
            addressbook::contract_address("weth", self.config.chain).ok_or_else(|| {
                format!(
                    "Contract not deployed on {}: {}",
                    self.network, self.config.chain
                )
            })?;

        let weth_contract = WETH::new(weth_address, self.config.http.clone());

        let amount = U256::from(amount);
        let call = weth_contract.deposit().value(amount);

        let tx = call.send().await?;
        println!("Transaction sent: {:?}", tx.tx_hash());

        let receipt = tx.await?;
        println!("Transaction receipt: {:?}", receipt);

        Ok(())
    }

    pub async fn send_eth(
        &self,
        to_address: String,
        amount: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let to_address = Address::from_str(&to_address)?;
        let amount = U256::from(amount);

        let tx = TransactionRequest::new()
            .to(to_address)
            .value(amount)
            .from(self.config.http.address());

        let pending_tx = self.config.http.send_transaction(tx, None).await?;
        println!("Transaction sent: {:?}", pending_tx.tx_hash());

        let receipt = pending_tx.await?;
        println!("Transaction receipt: {:?}", receipt);

        Ok(())
    }

    pub async fn send_erc20(
        &self,
        token_address: String,
        to_address: String,
        amount: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token_address = Address::from_str(&token_address)?;
        let to_address = Address::from_str(&to_address)?;
        let amount = U256::from(amount);

        let token = ERC20::new(token_address, self.config.http.clone());
        let tx = token.transfer(to_address, amount);

        let pending_tx = tx.send().await?;
        println!("Transaction sent: {:?}", pending_tx.tx_hash());

        let receipt = pending_tx.await?;
        println!("Transaction receipt: {:?}", receipt);

        Ok(())
    }
}
