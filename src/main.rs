mod config;
mod encoder;
mod utils; // Add this line

use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{Http, Provider, Ws};
use ethers::types::{Address, Filter, H256, U64};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::GetBlockNumber(args) => {
            handle_get_block_number(args.network).await?;
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
