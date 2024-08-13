use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::{Address, Filter, H256, U64};
use openzeppelin_rs::ERC20;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::bindings::weth::WETH;
use crate::config::{get_chain_config, get_chain_from_string};
use crate::{addressbook, utils};

pub async fn get_block_number(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let block_number: U64 = chain_config.http.get_block_number().await?;
    println!("Current block number on {}: {block_number}", network);
    Ok(())
}

pub async fn subscribe_blocks(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let mut stream = chain_config.ws.subscribe_blocks().await?;
    println!("Subscribing to new blocks on {}...", network);
    while let Some(block) = stream.next().await {
        println!("New block: {:?}", block.number.unwrap());
    }

    Ok(())
}

pub async fn subscribe_pending_transactions(
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let mut stream = chain_config.ws.subscribe_pending_txs().await?;
    println!("Subscribing to pending transactions on {}...", network);
    while let Some(tx_hash) = stream.next().await {
        println!("Pending transaction: {:?}", tx_hash);
    }

    Ok(())
}

pub async fn get_gas_price(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let gas_price = chain_config.http.get_gas_price().await?;
    println!("Current gas price on {}: {} wei", network, gas_price);
    println!(
        "Current gas price on {}: {} gwei",
        network,
        gas_price.as_u64() as f64 / 1_000_000_000.0
    );

    Ok(())
}

pub async fn get_balance(
    address: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;
    let address = Address::from_str(&address)?;
    let balance = chain_config.http.get_balance(address, None).await?;

    println!("Balance of {} on {}: {} wei", address, network, balance);
    println!(
        "Balance in ETH: {} ETH",
        ethers::utils::format_ether(balance)
    );
    Ok(())
}

pub async fn get_nonce(address: String, network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let address = Address::from_str(&address)?;
    let nonce = chain_config
        .http
        .get_transaction_count(address, None)
        .await?;

    println!("Nonce for address {} on {}: {}", address, network, nonce);
    Ok(())
}

pub async fn get_block_details(
    block_number: u64,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let block = chain_config.http.get_block(block_number).await?;

    match block {
        Some(block) => {
            println!("Block details for block {} on {}:", block_number, network);
            utils::print_block_details(&block);
        }
        None => println!("Block {} not found on {}", block_number, network),
    }

    Ok(())
}

pub async fn subscribe_logs(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let mut stream = chain_config.ws.subscribe_logs(&Filter::new()).await?;
    println!("Subscribing to logs on {}...", network);
    while let Some(log) = stream.next().await {
        println!("New log: {:?}", log);
    }

    Ok(())
}

pub async fn get_tx_details(
    tx_hash: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;
    let tx_hash = H256::from_str(&tx_hash)?;
    let tx = chain_config.http.get_transaction(tx_hash).await?;

    match tx {
        Some(tx) => {
            println!("Transaction details for {:?} on {}:", tx_hash, network);
            utils::print_tx_details(&tx);
        }
        None => println!("Transaction {:?} not found on {}", tx_hash, network),
    }

    Ok(())
}

pub async fn generate_contract_bindings(
    contract_address: String,
    contract_name: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Generating contract bindings for {} on {}",
        contract_address, network
    );

    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let client = Client::new(chain, chain_config.explorer_api_key)?;

    println!(
        "Fetching ABI for contract {} on {}...",
        contract_address, network
    );

    let abi: Abi = client.contract_abi(contract_address.parse()?).await?;
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
    contract_address: String,
    contract_name: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Downloading contract and generating bindings for {} on {}",
        contract_address, network
    );

    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let client = Client::new(chain, chain_config.explorer_api_key)?;

    println!(
        "Downloading ABI and source code for contract {}...",
        contract_address
    );
    let source_code = client
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
        let source_file = bindings_dir.join(format!("{}_source.sol", contract_name.to_lowercase()));
        println!("Saving source code to {:?}", source_file);
        fs::write(&source_file, contract.source_code())?;

        // Generate bindings
        println!("Generating bindings for {}...", contract.contract_name);
        let output_file = bindings_dir.join(format!("{}.rs", contract_name.to_lowercase()));
        let bindings = Abigen::new(&contract.contract_name, contract.abi.clone())?.generate()?;
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
    network: String,
    wallet_address: String,
    token_address: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let token_address = Address::from_str(&token_address)?;
    let wallet_address = Address::from_str(&wallet_address)?;
    let token = ERC20::new(token_address, chain_config.http);
    let balance = token.balance_of(wallet_address).await?;

    println!("Balance of {} on {}: {}", wallet_address, network, balance);
    Ok(())
}

pub async fn wrap_eth(amount: u64, network: String) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let chain_config = get_chain_config(chain).await;

    let weth_address = addressbook::contract_address("weth", chain)
        .ok_or_else(|| format!("Contract not deployed on {}: {}", network, chain))?;

    let weth_contract = WETH::new(weth_address, chain_config.http.clone());

    let amount = U256::from(amount);
    let call = weth_contract.deposit().value(amount);

    let tx = call.send().await?;
    println!("Transaction sent: {:?}", tx.tx_hash());

    let receipt = tx.await?;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}

pub async fn send_eth(
    to_address: String,
    amount: u64,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let to_address = Address::from_str(&to_address)?;
    let amount = U256::from(amount);

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .from(chain_config.http.address());

    let pending_tx = chain_config.http.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());

    let receipt = pending_tx.await?;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}

pub async fn send_erc20(
    token_address: String,
    to_address: String,
    amount: u64,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chain = get_chain_from_string(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;
    let chain_config = get_chain_config(chain).await;

    let token_address = Address::from_str(&token_address)?;
    let to_address = Address::from_str(&to_address)?;
    let amount = U256::from(amount);

    let token = ERC20::new(token_address, chain_config.http.clone());
    let tx = token.transfer(to_address, amount);

    let pending_tx = tx.send().await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());

    let receipt = pending_tx.await?;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}
