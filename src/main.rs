mod addressbook;
mod bindings;
mod config;
mod encoder;
mod handlers;
mod signer_middleware;
mod utils;

use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::types::U256;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::GetBlockNumber(args) => {
            let network = args.network;
            handlers::get_block_number(network).await?;
        }
        Command::SubscribeBlocks(args) => {
            handlers::subscribe_blocks(args.network).await?;
        }
        Command::SubscribePendingTransactions(args) => {
            handlers::subscribe_pending_transactions(args.network).await?;
        }
        Command::GetGasPrice(args) => {
            handlers::get_gas_price(args.network).await?;
        }
        Command::GetBalance(args) => {
            handlers::get_balance(args.address, args.network).await?;
        }
        Command::GetNonce(args) => {
            handlers::get_nonce(args.address, args.network).await?;
        }
        Command::GetBlockDetails(args) => {
            handlers::get_block_details(args.block_number, args.network).await?;
        }
        Command::SubscribeLogs(args) => {
            handlers::subscribe_logs(args.network).await?;
        }
        Command::GetTxDetails(args) => {
            handlers::get_tx_details(args.tx_hash, args.network).await?;
        }
        Command::GenerateContractBindings(args) => {
            println!(
                "Generating contract bindings for {} on {}",
                args.contract_address, args.network
            );
            handlers::generate_contract_bindings(
                args.contract_address,
                args.contract_name,
                args.network,
            )
            .await?;
        }
        Command::GenerateSourceCode(args) => {
            handlers::generate_source_code(args.contract_address, args.contract_name, args.network)
                .await?;
        }
        Command::GetERC20Balance(args) => {
            handlers::get_erc_20_balances(args.network, args.wallet_address, args.token_address)
                .await?;
        }
        Command::WrapETH(args) => {
            handlers::wrap_eth(args.amount, args.network).await?;
        }
        Command::SendETH(args) => {
            handlers::send_eth(args.to_address, args.amount, args.network).await?;
        }
        Command::SendERC20(args) => {
            handlers::send_erc20(
                args.token_address,
                args.to_address,
                args.amount,
                args.network,
            )
            .await?;
        }
        _ => {
            println!("Unsupported command");
        }
    }

    Ok(())
}
