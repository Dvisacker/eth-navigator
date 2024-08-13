mod config;
mod encoder;
mod utils;

use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::providers::{Http, Provider, Ws};
use ethers::types::{Address, Filter, H256, U64};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, fs};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    GetBlockNumber(NetworkArgs),
    SubscribeBlocks(NetworkArgs),
    SubscribePendingTransactions(NetworkArgs),
    GetGasPrice(NetworkArgs),
    GetBalance(GetBalanceArgs),
    GetNonce(GetNonceArgs),
    GetBlockDetails(GetBlockDetailsArgs),
    SubscribeLogs(SubscribeLogsArgs),
    GetTxDetails(GetTxDetailsArgs),
    GenerateContractBindings(GenerateContractBindingsArgs),
    GenerateSourceCode(GenerateSourceCodeArgs),
}

#[derive(Args)]
struct NetworkArgs {
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetBalanceArgs {
    #[clap(long)]
    address: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetERC20BalanceArgs {
    #[clap(long)]
    address: String,
    #[clap(long)]
    token_address: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetNonceArgs {
    #[clap(long)]
    address: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetBlockDetailsArgs {
    #[clap(long)]
    block_number: u64,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct SubscribeLogsArgs {
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetTxDetailsArgs {
    #[clap(long)]
    tx_hash: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GenerateContractBindingsArgs {
    #[clap(long)]
    contract_address: String,
    #[clap(long)]
    contract_name: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GenerateSourceCodeArgs {
    #[clap(long)]
    contract_address: String,
    #[clap(long)]
    contract_name: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::GetBlockNumber(args) => {
            let network = args.network;
            handle_get_block_number(network).await?;
        }
        Command::SubscribeBlocks(args) => {
            handle_subscribe_blocks(args.network).await?;
        }
        Command::SubscribePendingTransactions(args) => {
            handle_subscribe_pending_transactions(args.network).await?;
        }
        Command::GetGasPrice(args) => {
            handle_get_gas_price(args.network).await?;
        }
        Command::GetBalance(args) => {
            handle_get_balance(args.address, args.network).await?;
        }
        Command::GetNonce(args) => {
            handle_get_nonce(args.address, args.network).await?;
        }
        Command::GetBlockDetails(args) => {
            handle_get_block_details(args.block_number, args.network).await?;
        }
        Command::SubscribeLogs(args) => {
            handle_subscribe_logs(args.network).await?;
        }
        Command::GetTxDetails(args) => {
            handle_get_tx_details(args.tx_hash, args.network).await?;
        }
        Command::GenerateContractBindings(args) => {
            println!(
                "Generating contract bindings for {} on {}",
                args.contract_address, args.network
            );
            handle_generate_contract_bindings(
                args.contract_address,
                args.contract_name,
                args.network,
            )
            .await?;
        }
        Command::GenerateSourceCode(args) => {
            handle_generate_source_code(args.contract_address, args.contract_name, args.network)
                .await?;
        }
        _ => {
            println!("Unsupported command");
        }
    }

    Ok(())
}

async fn handle_get_block_number(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| network_config.rpc_url.clone());
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current block number on {}: {block_number}", network);
    Ok(())
}

async fn handle_subscribe_blocks(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let ws_url = env::var("WS_URL").unwrap_or_else(|_| network_config.ws_url.clone());
    let provider = Provider::<Ws>::connect(ws_url).await?;
    let provider = Arc::new(provider);

    let mut stream = provider.subscribe_blocks().await?;
    println!("Subscribing to new blocks on {}...", network);
    while let Some(block) = stream.next().await {
        println!("New block: {:?}", block.number.unwrap());
    }

    Ok(())
}

async fn handle_subscribe_pending_transactions(
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let ws_url = env::var("WS_URL").unwrap_or_else(|_| network_config.ws_url.clone());
    let provider = Provider::<Ws>::connect(ws_url).await?;
    let provider = Arc::new(provider);

    let mut stream = provider.subscribe_pending_txs().await?;
    println!("Subscribing to pending transactions on {}...", network);
    while let Some(tx_hash) = stream.next().await {
        println!("Pending transaction: {:?}", tx_hash);
    }

    Ok(())
}

async fn handle_get_gas_price(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| network_config.rpc_url.clone());
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let gas_price = provider.get_gas_price().await?;
    println!("Current gas price on {}: {} wei", network, gas_price);
    println!(
        "Current gas price on {}: {} gwei",
        network,
        gas_price.as_u64() as f64 / 1_000_000_000.0
    );

    Ok(())
}

async fn handle_get_balance(
    address: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let provider = Provider::<Http>::try_from(network_config.rpc_url.clone())?;
    let address = Address::from_str(&address)?;
    let balance = provider.get_balance(address, None).await?;

    println!("Balance of {} on {}: {} wei", address, network, balance);
    println!(
        "Balance in ETH: {} ETH",
        ethers::utils::format_ether(balance)
    );
    Ok(())
}

async fn handle_get_nonce(
    address: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let provider = Provider::<Http>::try_from(network_config.rpc_url.clone())?;
    let address = Address::from_str(&address)?;
    let nonce = provider.get_transaction_count(address, None).await?;

    println!("Nonce for address {} on {}: {}", address, network, nonce);
    Ok(())
}

async fn handle_get_block_details(
    block_number: u64,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let provider = Provider::<Http>::try_from(network_config.rpc_url.clone())?;
    let block = provider.get_block(block_number).await?;

    match block {
        Some(block) => {
            println!("Block details for block {} on {}:", block_number, network);
            utils::print_block_details(&block);
        }
        None => println!("Block {} not found on {}", block_number, network),
    }

    Ok(())
}

async fn handle_subscribe_logs(network: String) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let ws_provider = Provider::<Ws>::connect(network_config.ws_url.clone()).await?;
    let ws_provider = Arc::new(ws_provider);

    let mut stream = ws_provider.subscribe_logs(&Filter::new()).await?;
    println!("Subscribing to logs on {}...", network);
    while let Some(log) = stream.next().await {
        println!("New log: {:?}", log);
    }

    Ok(())
}

async fn handle_get_tx_details(
    tx_hash: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let provider = Provider::<Http>::try_from(network_config.rpc_url.clone())?;
    let tx_hash = H256::from_str(&tx_hash)?;

    let tx = provider.get_transaction(tx_hash).await?;

    match tx {
        Some(tx) => {
            println!("Transaction details for {:?} on {}:", tx_hash, network);
            utils::print_tx_details(&tx);
        }
        None => println!("Transaction {:?} not found on {}", tx_hash, network),
    }

    Ok(())
}

async fn handle_generate_contract_bindings(
    contract_address: String,
    contract_name: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Generating contract bindings for {} on {}",
        contract_address, network
    );

    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let chain = Chain::try_from(network_config.chain_id).unwrap();
    let api_key = network_config.explorer_api_key.to_string();
    let client = Client::new(chain, api_key)?;

    println!(
        "Fetching ABI for contract {} on {}...",
        contract_address, network
    );

    let abi: Abi = client.contract_abi(contract_address.parse()?).await?;
    let abi_str = serde_json::to_string(&abi)?;
    let bindings_dir = Path::new("bindings");
    fs::create_dir_all(bindings_dir)?;

    let output_file = bindings_dir.join(format!("{}.rs", contract_name.to_lowercase()));
    let bindings = Abigen::new(&contract_name, abi_str)?.generate()?;

    fs::write(&output_file, bindings.to_string())?;
    println!("Bindings generated and saved to {:?}", output_file);

    Ok(())
}

async fn handle_generate_source_code(
    contract_address: String,
    contract_name: String,
    network: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Downloading contract and generating bindings for {} on {}",
        contract_address, network
    );

    let network_config = config::get_network_config(&network)
        .ok_or_else(|| format!("Unsupported network: {}", network))?;

    let chain = Chain::try_from(network_config.chain_id).unwrap();
    let api_key = network_config.explorer_api_key.to_string();
    let client = Client::new(chain, api_key)?;

    println!(
        "Downloading ABI and source code for contract {}...",
        contract_address
    );
    let source_code = client
        .contract_source_code(contract_address.parse()?)
        .await?;

    let bindings_dir = Path::new("bindings");
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
