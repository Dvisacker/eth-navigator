use crate::bridge::lifi_types::{LifiChain, LifiConnection};
use crate::bridge::{LifiRoute, LifiToken};
use ethers::types::{Block, H256, U256};
use ethers::types::{Transaction, U64};
use prettytable::{Cell, Row, Table};

pub fn format_terminal_link(url: &str, text: &str) -> String {
    format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", url, text)
}

pub fn format_explorer_address_abbreviated(explorer_link: &str, address: &str) -> String {
    format_terminal_link(&format!("{}/address/{}", explorer_link, address), &address)
}

pub fn format_explorer_address_link(explorer_link: &str, address: &str) -> String {
    format!("{}/address/{}", explorer_link, address)
}

pub fn abbreviate_address(address: &str) -> String {
    format!("{}...{}", &address[..6], &address[address.len() - 4..])
}

pub fn print_block_details(block: &Block<H256>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Field").style_spec("b"),
        Cell::new("Value").style_spec("b"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Hash"),
        Cell::new(&format!("{:?}", block.hash)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Parent Hash"),
        Cell::new(&format!("{:?}", block.parent_hash)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Number"),
        Cell::new(&format!("{:?}", block.number)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Timestamp"),
        Cell::new(&format!("{:?}", block.timestamp)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Nonce"),
        Cell::new(&format!("{:?}", block.nonce)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Difficulty"),
        Cell::new(&format!("{:?}", block.difficulty)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Gas Limit"),
        Cell::new(&format!("{:?}", block.gas_limit)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Gas Used"),
        Cell::new(&format!("{:?}", block.gas_used)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Base Fee Per Gas"),
        Cell::new(&format!("{:?}", block.base_fee_per_gas)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Transactions"),
        Cell::new(&format!("{} transactions", block.transactions.len())),
    ]));

    table.printstd();
}

pub fn print_tx_details(tx: &Transaction) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Field").style_spec("b"),
        Cell::new("Value").style_spec("b"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Hash"),
        Cell::new(&format!("{:?}", tx.hash)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("From"),
        Cell::new(&format!("{:?}", tx.from)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("To"),
        Cell::new(&format!("{:?}", tx.to)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Nonce"),
        Cell::new(&format!("{:?}", tx.nonce)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Value"),
        Cell::new(&format!("{:?}", tx.value)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Gas Price"),
        Cell::new(&format!("{:?}", tx.gas_price)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Gas"),
        Cell::new(&format!("{:?}", tx.gas)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Link"),
        Cell::new(&format_terminal_link(
            &format!("https://etherscan.io/tx/{:?}", tx.hash),
            "Etherscan",
        )),
    ]));

    table.printstd();
}

pub fn print_lifi_chains(chains: &[LifiChain]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Key").style_spec("b"),
        Cell::new("Name").style_spec("b"),
        Cell::new("Chain Type").style_spec("b"),
        Cell::new("Coin").style_spec("b"),
        Cell::new("ID").style_spec("b"),
        Cell::new("Explorer").style_spec("b"),
        Cell::new("Multicall Address").style_spec("b"),
    ]));

    for chain in chains {
        let explorer_link = chain
            .metamask
            .block_explorer_urls
            .first()
            .map(String::as_str)
            .unwrap_or("");

        let multicall_link = &format_explorer_address_link(explorer_link, &chain.multicall_address);

        table.add_row(Row::new(vec![
            Cell::new(&chain.key),
            Cell::new(&chain.name),
            Cell::new(&chain.chain_type),
            Cell::new(&chain.coin),
            Cell::new(&chain.id.to_string()),
            Cell::new(&explorer_link.to_string()),
            Cell::new(&multicall_link.to_string()),
        ]));
    }

    table.printstd();
}

pub fn print_lifi_tokens(tokens: &[LifiToken]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Chain ID").style_spec("b"),
        Cell::new("Address").style_spec("b"),
        Cell::new("Symbol").style_spec("b"),
        Cell::new("Decimals").style_spec("b"),
        Cell::new("Price USD").style_spec("b"),
        Cell::new("Coin Key").style_spec("b"),
    ]));

    for token in tokens {
        table.add_row(Row::new(vec![
            Cell::new(&token.chain_id.to_string()),
            Cell::new(&token.address),
            Cell::new(&token.symbol),
            Cell::new(&token.decimals.to_string()),
            Cell::new(&token.price_USD),
            Cell::new(&token.coin_key.as_deref().unwrap_or("N/A")),
        ]));
    }

    table.printstd();
}

pub fn print_lifi_connections(connections: &[LifiConnection]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("From Chain ID").style_spec("b"),
        Cell::new("To Chain ID").style_spec("b"),
        Cell::new("From Tokens").style_spec("b"),
        Cell::new("To Tokens").style_spec("b"),
    ]));

    for connection in connections {
        let from_tokens = connection
            .from_tokens
            .iter()
            .map(|t| t.symbol.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let to_tokens = connection
            .to_tokens
            .iter()
            .map(|t| t.symbol.clone())
            .collect::<Vec<_>>()
            .join(", ");
        table.add_row(Row::new(vec![
            Cell::new(&connection.from_chain_id.to_string()),
            Cell::new(&connection.to_chain_id.to_string()),
            Cell::new(&from_tokens),
            Cell::new(&to_tokens),
        ]));
    }

    table.printstd();
}

pub fn print_routes(routes: &[LifiRoute]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Id").style_spec("b"),
        Cell::new("From Chain ID").style_spec("b"),
        Cell::new("To Chain ID").style_spec("b"),
        Cell::new("From Token").style_spec("b"),
        Cell::new("To Token").style_spec("b"),
        Cell::new("From Amount").style_spec("b"),
        Cell::new("To Amount").style_spec("b"),
        Cell::new("Step Bridges").style_spec("b"),
        Cell::new("Step Types").style_spec("b"),
        Cell::new("Gas Cost USD").style_spec("b"),
    ]));

    for route in routes {
        let tools = route
            .steps
            .iter()
            .map(|s| s.tool.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let types = route
            .steps
            .iter()
            .map(|s| s.step_type.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let gas_cost_usd = route.gas_cost_usd.as_deref().unwrap_or("N/A");
        table.add_row(Row::new(vec![
            Cell::new(&route.id),
            Cell::new(&route.from_chain_id.to_string()),
            Cell::new(&route.to_chain_id.to_string()),
            Cell::new(&route.from_token.symbol),
            Cell::new(&route.to_token.symbol),
            Cell::new(&route.from_amount.to_string()),
            Cell::new(&route.to_amount.to_string()),
            Cell::new(&tools),
            Cell::new(&types),
            Cell::new(&gas_cost_usd),
        ]));
    }

    table.printstd();
}

pub fn print_txs(transactions: &[Transaction]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Hash").style_spec("b"),
        Cell::new("Block Number").style_spec("b"),
        Cell::new("From").style_spec("b"),
        Cell::new("To").style_spec("b"),
        Cell::new("Value").style_spec("b"),
        Cell::new("Gas").style_spec("b"),
        Cell::new("Gas Price").style_spec("b"),
    ]));

    for tx in transactions {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{:?}", tx.hash)),
            Cell::new(&tx.block_number.unwrap_or(U64::zero()).to_string()),
            Cell::new(&format!("{:?}", tx.from)),
            Cell::new(&format!("{:?}", tx.to)),
            Cell::new(&tx.value.to_string()),
            Cell::new(&tx.gas.to_string()),
            Cell::new(&tx.gas_price.unwrap_or(U256::zero()).to_string()),
        ]));
    }

    table.printstd();
}
