use ethers::abi::{Abi, Token};
use ethers::prelude::*;
use ethers::types::{Address, Filter, H256, U64};
use openzeppelin_rs::ERC20;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bindings::uniswap_v2_factory::UNISWAP_V2_FACTORY;
use crate::bindings::uniswap_v2_pool::UNISWAP_V2_POOL;
use crate::bindings::uniswap_v2_router::UNISWAP_V2_ROUTER;
use crate::bindings::uniswap_v3_router::{ExactInputParams, UNISWAP_V3_ROUTER};
use crate::bindings::weth::WETH;
use crate::config::{get_chain_config, get_chain_from_string, ChainConfig};
use crate::whitelist::Whitelist;
use crate::{addressbook, utils};

pub struct EVMInterface {
    config: Arc<ChainConfig>,
    explorer_client: Arc<Client>,
    network: String,
    whitelist: Arc<Whitelist>,
}

impl EVMInterface {
    pub async fn new(
        network: &str,
        whitelist: Arc<Whitelist>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let chain = get_chain_from_string(network)
            .ok_or_else(|| format!("Unsupported network: {}", network))?;
        let config = get_chain_config(chain).await;
        let explorer_api_key = config.explorer_api_key.clone();
        let explorer_client = Client::new(chain, explorer_api_key)?;
        Ok(Self {
            config: Arc::new(config),
            network: network.to_string(),
            explorer_client: Arc::new(explorer_client),
            whitelist,
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

        if !self
            .whitelist
            .is_wallet_whitelisted(&to_address.to_string())
        {
            return Err("Recipient address is not whitelisted".into());
        }

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

        if !self
            .whitelist
            .is_token_whitelisted(&token_address.to_string(), self.config.chain_id)
        {
            return Err("Token address is not whitelisted".into());
        }

        if !self
            .whitelist
            .is_wallet_whitelisted(&to_address.to_string())
        {
            return Err("Recipient address is not whitelisted".into());
        }

        let amount = U256::from(amount);

        let token = ERC20::new(token_address, self.config.http.clone());
        let tx = token.transfer(to_address, amount);

        let pending_tx = tx.send().await?;
        println!("Transaction sent: {:?}", pending_tx.tx_hash());

        let receipt = pending_tx.await?;
        println!("Transaction receipt: {:?}", receipt);

        Ok(())
    }

    pub async fn get_transactions(
        &self,
        address: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let address = Address::from_str(&address)?;
        let transactions = self.find_all_transactions(address).await?;

        println!("Transactions for address {} on {}:", address, self.network);
        utils::print_txs(&transactions);

        Ok(())
    }

    async fn find_all_transactions(
        &self,
        address: Address,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let mut end = self.config.http.get_block_number().await?;
        let mut start = end - 1000000;

        while start <= end {
            let mid = (start + end) / 2;
            if self.has_transactions(address, mid).await? {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        let first_block = start;
        let mut transactions = Vec::new();

        // Exponential search forward
        let mut block = first_block;
        let mut step = U64::from(1);
        loop {
            let block_txs = self.get_address_transactions(address, block).await?;
            if block_txs.is_empty() {
                // Binary search in the last interval
                let last_with_tx = self
                    .binary_search_last_with_tx(address, block - step, block)
                    .await?;
                transactions.extend(
                    self.get_all_txs_in_range(address, first_block, last_with_tx)
                        .await?,
                );
                break;
            }
            transactions.extend(block_txs);
            block += step;
            step *= 2;
        }

        Ok(transactions.into_iter().collect())
    }

    async fn has_transactions(
        &self,
        address: Address,
        block_number: U64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let block = self.config.http.get_block_with_txs(block_number).await?;
        Ok(block.map_or(false, |b| {
            b.transactions
                .iter()
                .any(|tx| tx.from == address || tx.to == Some(address))
        }))
    }

    async fn get_address_transactions(
        &self,
        address: Address,
        block_number: U64,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let block = self.config.http.get_block_with_txs(block_number).await?;
        Ok(block.map_or(Vec::new(), |b| {
            b.transactions
                .into_iter()
                .filter(|tx| tx.from == address || tx.to == Some(address))
                .collect()
        }))
    }

    async fn binary_search_last_with_tx(
        &self,
        address: Address,
        mut start: U64,
        mut end: U64,
    ) -> Result<U64, Box<dyn std::error::Error>> {
        while start <= end {
            let mid = (start + end) / 2;
            if self.has_transactions(address, mid).await? {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        Ok(end)
    }

    async fn get_all_txs_in_range(
        &self,
        address: Address,
        start_block: U64,
        end_block: U64,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let mut transactions = Vec::new();
        let start: u64 = start_block.as_u64();
        let end: u64 = end_block.as_u64();
        for block_number in start..=end {
            transactions.extend(
                self.get_address_transactions(address, U64::from(block_number))
                    .await?,
            );
        }
        Ok(transactions)
    }

    pub async fn swap_tokens_uniswap_v3(
        &self,
        token_in: String,
        token_out: String,
        amount_in: String,
        amount_out_minimum: String,
        recipient: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token_in = Address::from_str(&token_in)?;
        let token_out = Address::from_str(&token_out)?;
        let recipient = Address::from_str(&recipient)?;

        if !self
            .whitelist
            .is_token_whitelisted(&token_in.to_string(), self.config.chain_id)
        {
            return Err("Input token address is not whitelisted".into());
        }

        if !self
            .whitelist
            .is_token_whitelisted(&token_out.to_string(), self.config.chain_id)
        {
            return Err("Output token address is not whitelisted".into());
        }

        if !self.whitelist.is_wallet_whitelisted(&recipient.to_string()) {
            return Err("Recipient address is not whitelisted".into());
        }

        // Create ERC20 instances for token_in and token_out
        let token_in_contract = ERC20::new(token_in, self.config.http.clone());
        let token_out_contract = ERC20::new(token_out, self.config.http.clone());

        // Get token decimals
        let token_in_decimals = token_in_contract.decimals().call().await?;
        let token_out_decimals = token_out_contract.decimals().call().await?;

        // Parse amounts considering token decimals
        let amount_in =
            U256::from_dec_str(&amount_in)? * U256::from(10).pow(U256::from(token_in_decimals));
        let amount_out_minimum = U256::from_dec_str(&amount_out_minimum)?
            * U256::from(10).pow(U256::from(token_out_decimals));

        let uniswap_router_address =
            addressbook::contract_address("uniswap_v3_router", self.config.chain)
                .ok_or_else(|| format!("Uniswap V3 Router not deployed on {}", self.network))?;

        // Approve token_in for Uniswap router
        let approve_tx = token_in_contract.approve(uniswap_router_address, amount_in);
        let pending_approve_tx = approve_tx.send().await?;
        println!(
            "Approval transaction sent: {:?}",
            pending_approve_tx.tx_hash()
        );
        let approve_receipt = pending_approve_tx.await?;
        println!("Approval transaction receipt: {:?}", approve_receipt);

        let uniswap_router =
            UNISWAP_V3_ROUTER::new(uniswap_router_address, self.config.http.clone());

        let path = ethers::abi::encode(&[
            Token::Address(token_in),
            Token::Uint(U256::from(3000)),
            Token::Address(token_out),
        ]);

        let params = ExactInputParams(
            Bytes::from(path),
            recipient,
            U256::from(u64::MAX), // Set a reasonable deadline
            amount_in,
            amount_out_minimum,
        );

        let tx = uniswap_router.exact_input(params);
        let pending_tx = tx.send().await?;
        println!("Swap transaction sent: {:?}", pending_tx.tx_hash());

        let receipt = pending_tx.await?;
        println!("Swap transaction receipt: {:?}", receipt);

        Ok(())
    }

    pub async fn add_liquidity_uniswap_v2(
        &self,
        token_a: String,
        token_b: String,
        amount_a_desired: String,
        amount_a_min: String,
        to: String,
        deadline: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token_a = Address::from_str(&token_a)?;
        let token_b = Address::from_str(&token_b)?;
        let to = Address::from_str(&to)?;

        if !self
            .whitelist
            .is_token_whitelisted(&token_a.to_string(), self.config.chain_id)
        {
            return Err("Token A address is not whitelisted".into());
        }

        if !self
            .whitelist
            .is_token_whitelisted(&token_b.to_string(), self.config.chain_id)
        {
            return Err("Token B address is not whitelisted".into());
        }

        if !self.whitelist.is_wallet_whitelisted(&to.to_string()) {
            return Err("Recipient address is not whitelisted".into());
        }

        let token_a_contract = ERC20::new(token_a, self.config.http.clone());
        let token_b_contract = ERC20::new(token_b, self.config.http.clone());

        let token_a_decimals = token_a_contract.decimals().call().await?;
        let token_b_decimals = token_b_contract.decimals().call().await?;

        let amount_a_desired = U256::from_dec_str(&amount_a_desired)?
            * U256::from(10).pow(U256::from(token_a_decimals));
        let amount_a_min =
            U256::from_dec_str(&amount_a_min)? * U256::from(10).pow(U256::from(token_a_decimals));

        let uniswap_factory_address =
            addressbook::contract_address("uniswap_v2_factory", self.config.chain)
                .ok_or_else(|| format!("Uniswap V2 Factory not deployed on {}", self.network))?;

        let uniswap_factory =
            UNISWAP_V2_FACTORY::new(uniswap_factory_address, self.config.http.clone());

        // Get the pair address
        let pair_address = uniswap_factory.get_pair(token_a, token_b).call().await?;

        if pair_address == Address::zero() {
            return Err("Liquidity pool does not exist for the given token pair".into());
        }

        let pair_contract = UNISWAP_V2_POOL::new(pair_address, self.config.http.clone());

        // Get current reserves
        let (reserve_a, reserve_b, _) = pair_contract.get_reserves().call().await?;

        // Calculate amount_b based on the current price in the pool
        let amount_b_desired = if reserve_a == 0 || reserve_b == 0 {
            amount_a_desired // If the pool is empty, use the same amount as token A
        } else {
            amount_a_desired * U256::from(reserve_b) / U256::from(reserve_a)
        };

        let amount_b_min = amount_b_desired * 95 / 100; // Set amount_b_min to 95% of amount_b_desired

        let uniswap_router_address =
            addressbook::contract_address("uniswap_v2_router", self.config.chain)
                .ok_or_else(|| format!("Uniswap V2 Router not deployed on {}", self.network))?;

        let uniswap_router =
            UNISWAP_V2_ROUTER::new(uniswap_router_address, self.config.http.clone());

        // Approve token A
        let approve_a_tx = token_a_contract.approve(uniswap_router_address, amount_a_desired);
        let pending_approve_a_tx = approve_a_tx.send().await?;
        println!(
            "Approval transaction for token A sent: {:?}",
            pending_approve_a_tx.tx_hash()
        );
        let approve_a_receipt = pending_approve_a_tx.await?;
        println!(
            "Approval transaction receipt for token A: {:?}",
            approve_a_receipt
        );

        // Approve token B
        let approve_b_tx = token_b_contract.approve(uniswap_router_address, amount_b_desired);
        let pending_approve_b_tx = approve_b_tx.send().await?;
        println!(
            "Approval transaction for token B sent: {:?}",
            pending_approve_b_tx.tx_hash()
        );
        let approve_b_receipt = pending_approve_b_tx.await?;
        println!(
            "Approval transaction receipt for token B: {:?}",
            approve_b_receipt
        );

        let deadline = if deadline == 0 {
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600 // Default to 1 hour from now
        } else {
            deadline
        };

        let tx = uniswap_router.add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            to,
            deadline.into(),
        );
        let pending_tx = tx.send().await?;
        println!("Add liquidity transaction sent: {:?}", pending_tx.tx_hash());

        let receipt = pending_tx.await?;
        println!("Add liquidity transaction receipt: {:?}", receipt);

        Ok(())
    }
}
