use crate::bridge::lifi_types::LifiChain;
use ethers::types::Transaction;
use ethers::types::{Block, H256};
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
