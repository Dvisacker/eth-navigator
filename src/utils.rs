use ethers::types::{Block, H256};
use prettytable::{Cell, Row, Table};

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

use ethers::types::Transaction;

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
        Cell::new("Input"),
        Cell::new(&format!("{:?}", tx.input)),
    ]));

    table.printstd();
}
