mod addressbook;
mod bindings;
mod bridge;
mod config;
mod encoder;
mod evm_interface;
mod signer_middleware;
mod ui;
mod utils;
mod whitelist;
use crate::config::{
    get_chain_config, get_chain_from_string, get_chain_id_from_string, get_whitelist_path,
};
use crate::utils::{print_lifi_chains, print_lifi_connections, print_lifi_tokens, print_routes};
use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use evm_interface::EVMInterface;
use whitelist::Whitelist;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::GetBlockNumber(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_block_number().await?;
        }
        Command::SubscribeBlocks(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.subscribe_blocks().await?;
        }
        Command::SubscribePendingTransactions(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.subscribe_pending_transactions().await?;
        }
        Command::GetGasPrice(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_gas_price().await?;
        }
        Command::GetBalance(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_balance(args.address).await?;
        }
        Command::GetNonce(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_nonce(args.address).await?;
        }
        Command::GetBlockDetails(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_block_details(args.block_number).await?;
        }
        Command::SubscribeLogs(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.subscribe_logs().await?;
        }
        Command::GetTxDetails(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_tx_details(args.tx_hash).await?;
        }
        Command::GenerateContractBindings(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            println!(
                "Generating contract bindings for {} on {}",
                args.contract_address, args.network
            );
            evm_interface
                .generate_contract_bindings(args.contract_address, args.contract_name)
                .await?;
        }
        Command::GenerateSourceCode(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface
                .generate_source_code(args.contract_address, args.contract_name)
                .await?;
        }
        Command::GetERC20Balance(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface
                .get_erc_20_balances(args.wallet_address, args.token_address)
                .await?;
        }
        Command::WrapETH(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.wrap_eth(args.amount).await?;
        }
        Command::SendETH(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.send_eth(args.to_address, args.amount).await?;
        }
        Command::SendERC20(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface
                .send_erc20(args.token_address, args.to_address, args.amount)
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
            let request = bridge::lifi_types::RouteRequest::new(
                args.from_chain_id,
                args.to_chain_id,
                args.from_token_address,
                args.to_token_address,
                args.from_amount,
                // args.from_address,
                // args.to_address,
            );
            let routes = bridge.request_routes(request).await?;
            print_routes(&routes);
        }
        Command::RequestQuote(args) => {
            let bridge = bridge::lifi::LiFiBridge::new();
            let request = bridge::lifi_types::QuoteRequest::new(
                args.from_chain,
                args.to_chain,
                args.from_token,
                args.to_token,
                args.from_amount,
                args.from_address,
                args.to_address,
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
            let request = bridge::lifi_types::ConnectionsRequest::new(
                args.from_chain,
                args.to_chain,
                args.from_token,
                args.to_token,
                args.from_amount,
                args.allow_exchanges,
            );
            let connections = bridge.get_connections(request).await?;
            print_lifi_connections(&connections);
        }
        Command::GetTransactions(args) => {
            let evm_interface = EVMInterface::new(&args.network).await?;
            evm_interface.get_transactions(args.address).await?;
        }
        Command::AddWalletToWhitelist(args) => {
            let mut whitelist = load_or_create_whitelist()?;
            whitelist.add_wallet_address(args.address, args.name);
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
            whitelist.add_token_address(args.address, config.chain_id, args.name, config.http);
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
