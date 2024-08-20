mod addressbook;
mod bindings;
mod bridge;
mod config;
mod evm_interface;
mod signer_middleware;
mod utils;
mod whitelist;
use crate::config::{
    get_chain_config, get_chain_from_string, get_chain_id_from_string, get_whitelist_path,
};
use crate::utils::{print_lifi_chains, print_lifi_connections, print_lifi_tokens, print_routes};
use crate::whitelist::Whitelist;
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::types::{Address, Chain};
use evm_interface::EVMInterface;
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
    GenerateContractBindings(GenerateContractBindingsArgs),
    GenerateSourceCode(GenerateSourceCodeArgs),
    WrapETH(WrapETHArgs),
    GetERC20Balance(GetERC20BalanceArgs),
    SendETH(SendETHArgs),
    SendERC20(SendERC20Args),
    GetSupportedChains,
    GetKnownTokens(GetKnownTokensArgs),
    RequestRoutes(RequestRoutesArgs),
    RequestQuote(RequestQuoteArgs),
    GetTransferStatus(GetTransferStatusArgs),
    GetConnections(GetConnectionsArgs),
    GetTransactions(GetTransactionsArgs),
    AddWalletToWhitelist(AddWalletToWhitelistArgs),
    RemoveWalletFromWhitelist(RemoveWalletFromWhitelistArgs),
    AddTokenToWhitelist(AddTokenToWhitelistArgs),
    RemoveTokenFromWhitelist(RemoveTokenFromWhitelistArgs),
    ShowWhitelist,
    SwapTokensUniswapV3(SwapTokensUniswapV3Args),
    AddLiquidityUniswapV2(AddLiquidityUniswapV2Args),
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
    wallet_address: String,
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

#[derive(Args)]
struct WrapETHArgs {
    #[clap(long)]
    amount: u64,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct SendETHArgs {
    #[clap(long)]
    to_address: String,
    #[clap(long)]
    amount: u64,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct SendERC20Args {
    #[clap(long)]
    token_address: String,
    #[clap(long)]
    to_address: String,
    #[clap(long)]
    amount: u64,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct GetKnownTokensArgs {
    #[clap(long)]
    chain: String,
}

#[derive(Args)]
struct RequestRoutesArgs {
    #[clap(long)]
    from_chain_id: u64,
    #[clap(long)]
    to_chain_id: u64,
    #[clap(long)]
    from_token_address: String,
    #[clap(long)]
    to_token_address: String,
    #[clap(long)]
    from_amount: String,
}

#[derive(Args)]
struct RequestQuoteArgs {
    #[clap(long)]
    from_chain: String,
    #[clap(long)]
    to_chain: String,
    #[clap(long)]
    from_token: String,
    #[clap(long)]
    to_token: String,
    #[clap(long)]
    from_amount: String,
    #[clap(long)]
    from_address: String,
    #[clap(long)]
    to_address: String,
}

#[derive(Args)]
struct GetTransferStatusArgs {
    #[clap(long)]
    tx_hash: String,
    #[clap(long)]
    from_chain: Option<String>,
    #[clap(long)]
    to_chain: Option<String>,
    #[clap(long)]
    bridge: Option<String>,
}

#[derive(Args)]
struct GetConnectionsArgs {
    #[clap(long)]
    from_chain: Option<String>,
    #[clap(long)]
    to_chain: Option<String>,
    #[clap(long)]
    from_token: Option<String>,
    #[clap(long)]
    to_token: Option<String>,
    #[clap(long)]
    from_amount: Option<String>,
    #[clap(long)]
    allow_exchanges: Option<bool>,
}

#[derive(Args)]
struct GetTransactionsArgs {
    #[clap(long)]
    address: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
    #[clap(long)]
    blocks: Option<u64>,
}

#[derive(Args)]
struct AddWalletToWhitelistArgs {
    #[clap(long)]
    address: String,
    #[clap(long)]
    name: Option<String>,
}

#[derive(Args)]
struct RemoveWalletFromWhitelistArgs {
    #[clap(long)]
    address: String,
    #[clap(long)]
    name: Option<String>,
}

#[derive(Args)]
struct AddTokenToWhitelistArgs {
    #[clap(long)]
    address: String,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    chain: String,
}

#[derive(Args)]
struct RemoveTokenFromWhitelistArgs {
    #[clap(long)]
    address: String,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    chain: String,
}

#[derive(Args)]
struct SwapTokensUniswapV3Args {
    #[clap(long)]
    token_in: String,
    #[clap(long)]
    token_out: String,
    #[clap(long)]
    amount_in: String,
    #[clap(long)]
    amount_out_minimum: String,
    #[clap(long)]
    recipient: String,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[derive(Args)]
struct AddLiquidityUniswapV2Args {
    #[clap(long)]
    token_a: String,
    #[clap(long)]
    token_b: String,
    #[clap(long)]
    amount_a_desired: String,
    #[clap(long)]
    amount_a_min: String,
    #[clap(long)]
    to: String,
    #[clap(long)]
    deadline: u64,
    #[clap(long, default_value = "ethereum")]
    network: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let whitelist = Arc::new(load_or_create_whitelist()?);

    match cli.command {
        Command::GetBlockNumber(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.get_block_number().await?;
        }
        Command::SubscribeBlocks(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.subscribe_blocks().await?;
        }
        Command::SubscribePendingTransactions(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.subscribe_pending_transactions().await?;
        }
        Command::GetGasPrice(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.get_gas_price().await?;
        }
        Command::GetBalance(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist.clone()).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let address = resolve_address(&args.address, chain, whitelist)?;
            evm_interface.get_balance(address.to_string()).await?;
        }
        Command::GetNonce(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let whitelist = Arc::clone(&whitelist);
            let address = resolve_address(&args.address, chain, whitelist)?;
            evm_interface.get_nonce(address.to_string()).await?;
        }
        Command::GetBlockDetails(args) => {
            let whitelist = Arc::clone(&whitelist);
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.get_block_details(args.block_number).await?;
        }
        Command::SubscribeLogs(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.subscribe_logs().await?;
        }
        Command::GetTxDetails(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.get_tx_details(args.tx_hash).await?;
        }
        Command::GenerateContractBindings(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let contract_address = resolve_address(&args.contract_address, chain, whitelist)?;
            println!(
                "Generating contract bindings for {} on {}",
                contract_address, args.network
            );
            evm_interface
                .generate_contract_bindings(contract_address.to_string(), args.contract_name)
                .await?;
        }
        Command::GenerateSourceCode(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let contract_address = resolve_address(&args.contract_address, chain, whitelist)?;
            evm_interface
                .generate_source_code(contract_address.to_string(), args.contract_name)
                .await?;
        }
        Command::GetERC20Balance(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let wallet_address =
                resolve_address(&args.wallet_address, chain, Arc::clone(&whitelist))?;
            let token_address =
                resolve_address(&args.token_address, chain, Arc::clone(&whitelist))?;
            evm_interface
                .get_erc_20_balances(wallet_address.to_string(), token_address.to_string())
                .await?;
        }
        Command::WrapETH(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist).await?;
            evm_interface.wrap_eth(args.amount).await?;
        }
        Command::SendETH(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let to_address = resolve_address(&args.to_address, chain, Arc::clone(&whitelist))?;
            evm_interface
                .send_eth(to_address.to_string(), args.amount)
                .await?;
        }
        Command::SendERC20(args) => {
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let token_address =
                resolve_address(&args.token_address, chain, Arc::clone(&whitelist))?;
            let to_address = resolve_address(&args.to_address, chain, Arc::clone(&whitelist))?;
            evm_interface
                .send_erc20(
                    token_address.to_string(),
                    to_address.to_string(),
                    args.amount,
                )
                .await?;
        }
        Command::GetSupportedChains => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let chains = bridge.get_supported_chains().await?;
            print_lifi_chains(&chains);
        }
        Command::GetKnownTokens(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let tokens = bridge.get_known_tokens(&args.chain).await?;
            print_lifi_tokens(&tokens);
            // println!("Known tokens on {}: {:?}", args.chain, tokens);
        }
        Command::RequestRoutes(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let from_chain = get_chain_from_string(&args.from_chain_id.to_string()).unwrap();
            let to_chain = get_chain_from_string(&args.to_chain_id.to_string()).unwrap();
            let from_token_address =
                resolve_address(&args.from_token_address, from_chain, Arc::clone(&whitelist))?;
            let to_token_address =
                resolve_address(&args.to_token_address, to_chain, Arc::clone(&whitelist))?;
            let request = bridge::lifi_types::RouteRequest::new(
                args.from_chain_id,
                args.to_chain_id,
                from_token_address.to_string(),
                to_token_address.to_string(),
                args.from_amount,
                // args.from_address,
                // args.to_address,
            );
            let routes = bridge.request_routes(request).await?;
            print_routes(&routes);
        }
        Command::RequestQuote(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let from_chain = get_chain_from_string(&args.from_chain).unwrap();
            let to_chain = get_chain_from_string(&args.to_chain).unwrap();
            let from_token = resolve_address(&args.from_token, from_chain, Arc::clone(&whitelist))?;
            let to_token = resolve_address(&args.to_token, to_chain, Arc::clone(&whitelist))?;
            let from_address =
                resolve_address(&args.from_address, from_chain, Arc::clone(&whitelist))?;
            let to_address = resolve_address(&args.to_address, to_chain, Arc::clone(&whitelist))?;
            let request = bridge::lifi_types::QuoteRequest::new(
                args.from_chain,
                args.to_chain,
                from_token.to_string(),
                to_token.to_string(),
                args.from_amount,
                from_address.to_string(),
                to_address.to_string(),
            );
            let quote = bridge.request_quote(request).await?;
            println!("Quote: {:?}", quote);
        }
        Command::GetTransferStatus(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let request = bridge::lifi_types::StatusRequest::new(
                args.bridge,
                args.from_chain,
                args.to_chain,
                args.tx_hash,
            );
            let status = bridge.get_transfer_status(request).await?;
            println!("Transfer status: {:?}", status);
        }
        Command::GetConnections(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let from_chain = Chain::Mainnet;
            let to_chain = Chain::Mainnet;
            // let from_chain = get_chain_from_string(&args.from_chain).unwrap();
            // let to_chain = get_chain_from_string(&args.to_chain).unwrap();
            // let from_chain = args.from_chain.as_ref().map(|c| get_chain_from_string(c));
            // let to_chain = args.to_chain.as_ref().map(|c| get_chain_from_string(c));
            let from_token = args
                .from_token
                .as_ref()
                .map(|t| resolve_address(t, from_chain, Arc::clone(&whitelist)))
                .transpose()?;
            let to_token = args
                .to_token
                .as_ref()
                .map(|t| resolve_address(t, to_chain, Arc::clone(&whitelist)))
                .transpose()?;
            let request = bridge::lifi_types::ConnectionsRequest::new(
                args.from_chain,
                args.to_chain,
                from_token.map(|t| t.to_string()),
                to_token.map(|t| t.to_string()),
                args.from_amount,
                args.allow_exchanges,
            );
            let connections = bridge.get_connections(request).await?;
            print_lifi_connections(&connections);
        }
        Command::GetTransactions(args) => {
            let chain = get_chain_from_string(&args.network).unwrap();
            let evm_interface = EVMInterface::new(&args.network, Arc::clone(&whitelist)).await?;
            let address = resolve_address(&args.address, chain, Arc::clone(&whitelist))?;
            evm_interface.get_transactions(address.to_string()).await?;
        }
        Command::AddWalletToWhitelist(args) => {
            let mut whitelist = load_or_create_whitelist()?;
            whitelist.add_wallet_address(
                args.address.clone(),
                Some(args.name.clone().unwrap_or(args.address)),
            );
            whitelist.save(&get_whitelist_path().to_string_lossy())?;
            println!("Wallet address added to whitelist.");
        }
        Command::RemoveWalletFromWhitelist(args) => {
            let mut whitelist = load_or_create_whitelist()?;
            whitelist.remove_wallet_address(&args.address);
            whitelist.save(&get_whitelist_path().to_string_lossy())?;
            println!("Wallet address removed from whitelist.");
        }
        Command::AddTokenToWhitelist(args) => {
            let mut whitelist = load_or_create_whitelist()?;
            let chain = get_chain_from_string(&args.chain).unwrap();
            let config = get_chain_config(chain).await;
            whitelist
                .add_token_address(args.address, config.chain_id, args.name, config.http)
                .await?;
            whitelist.save(&get_whitelist_path().to_string_lossy())?;
            println!("Token address added to whitelist.");
        }
        Command::RemoveTokenFromWhitelist(args) => {
            let mut whitelist = load_or_create_whitelist()?;
            let chain_id = get_chain_id_from_string(&args.chain).unwrap();
            whitelist.remove_token_address(&args.address, chain_id);
            whitelist.save(&get_whitelist_path().to_string_lossy())?;
            println!("Token address removed from whitelist.");
        }
        Command::ShowWhitelist => {
            let whitelist = load_or_create_whitelist()?;
            println!("Whitelisted wallet addresses:");
            for (address, info) in whitelist.get_wallet_addresses() {
                println!(
                    "{} ({})",
                    address,
                    info.name.as_ref().unwrap_or(&String::new())
                );
            }
            println!("\nWhitelisted token addresses:");
            for (address, info) in whitelist.get_token_addresses() {
                println!("{}: {} on {}", info.symbol, address, info.chain_id);
            }
        }
        Command::SwapTokensUniswapV3(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist.clone()).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let token_in = resolve_address(&args.token_in, chain, Arc::clone(&whitelist))?;
            let token_out = resolve_address(&args.token_out, chain, Arc::clone(&whitelist))?;
            let recipient = resolve_address(&args.recipient, chain, Arc::clone(&whitelist))?;
            evm_interface
                .swap_tokens_uniswap_v3(
                    token_in.to_string(),
                    token_out.to_string(),
                    args.amount_in,
                    args.amount_out_minimum,
                    recipient.to_string(),
                )
                .await?;
        }
        Command::AddLiquidityUniswapV2(args) => {
            let evm_interface = EVMInterface::new(&args.network, whitelist.clone()).await?;
            let chain = get_chain_from_string(&args.network).unwrap();
            let token_a = resolve_address(&args.token_a, chain, Arc::clone(&whitelist))?;
            let token_b = resolve_address(&args.token_b, chain, Arc::clone(&whitelist))?;
            let to = resolve_address(&args.to, chain, Arc::clone(&whitelist))?;
            evm_interface
                .add_liquidity_uniswap_v2(
                    token_a.to_string(),
                    token_b.to_string(),
                    args.amount_a_desired,
                    args.amount_a_min,
                    to.to_string(),
                    args.deadline,
                )
                .await?;
        }
        _ => {
            println!("Unsupported command");
        }
    }

    Ok(())
}

fn load_or_create_whitelist() -> Result<Whitelist, Box<dyn std::error::Error>> {
    let path = get_whitelist_path();
    if path.exists() {
        Ok(Whitelist::load(&path.to_string_lossy())?)
    } else {
        Ok(Whitelist::new())
    }
}

fn resolve_address(
    input: &str,
    chain: Chain,
    whitelist: Arc<Whitelist>,
) -> Result<Address, Box<dyn std::error::Error>> {
    // Check if the input is a valid Ethereum address
    if let Ok(address) = Address::from_str(input) {
        return Ok(address);
    }

    // Check if the input is a whitelisted wallet name
    if let Some(wallet_info) = whitelist.get_wallet_by_name(input) {
        return Ok(Address::from_str(&wallet_info.address)?);
    }

    // Check if the input is a token name in the addressbook
    if let Some(address) = addressbook::contract_address(input, chain) {
        return Ok(address);
    }

    Err(format!("Could not resolve address for: {}", input).into())
}
