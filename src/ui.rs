use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::Value;
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

use crate::utils;

pub fn print_table() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Parse JSON
    let json_str = r#"{
        "chains": [
            {
                "id": 1,
                "key": "eth",
                "name": "Ethereum",
                "chainType": "EVM",
                "coin": "ETH",
                "mainnet": true,
                "logoURI": "https://raw.githubusercontent.com/lifinance/types/main/src/assets/icons/chains/ethereum.png",
                "tokenlistUrl": "https://gateway.ipfs.io/ipns/tokens.uniswap.org",
                "multicallAddress": "0x5BA1e12693Dc8F9c48aAD8770482f4739bEeD696",
                "metamask": {
                    "chainId": "0x1",
                    "blockExplorerUrls": [
                        "https://etherscan.io/"
                    ],
                    "chainName": "Ethereum Mainnet",
                    "nativeCurrency": {
                        "name": "ETH",
                        "symbol": "ETH",
                        "decimals": 18
                    },
                    "rpcUrls": [
                        "https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161"
                    ]
                },
                "nativeToken": {
                    "address": "0x0000000000000000000000000000000000000000",
                    "symbol": "ETH",
                    "decimals": 18,
                    "chainId": 1,
                    "name": "ETH",
                    "coinKey": "ETH",
                    "priceUSD": "1197.56",
                    "logoURI": "https://static.debank.com/image/token/logo_url/eth/935ae4e4d1d12d59a99717a24f2540b5.png"
                }
            }
        ]
    }"#;

    let json: Value = serde_json::from_str(json_str)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let table = create_table(&json);
            f.render_widget(table, size);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn create_table(json: &Value) -> Table {
    let chains = json["chains"].as_array().unwrap();

    let header_cells = [
        "ID",
        "Key",
        "Name",
        "Chain Type",
        "Coin",
        "Mainnet",
        "Multicall Address",
        "Native Token",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);

    let rows = chains.iter().map(|chain| {
        let multicall_address = chain["multicallAddress"].as_str().unwrap_or("");
        let etherscan_link = format!("https://etherscan.io/address/{:?}", multicall_address);
        let abbreviated_address = format!(
            "{}...{}",
            &multicall_address[..6],
            &multicall_address[multicall_address.len() - 4..]
        );

        let etherscan_link =
            "\x1B]8;;https://etherscan.io/address/0x5BA1e12693\x1B\\Link\x1B]8;;\x1B\\".to_string();
        // let etherscan_link2 = format!(
        //     "\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\",
        //     etherscan_link, abbreviated_address
        // );
        let etherscan_link2 = utils::format_terminal_link(&etherscan_link, "Link").to_string();
        // let etherscan_link2 = format!(
        //     "\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\",
        //     etherscan_link, abbreviated_address
        // );

        let native_token_address = chain["nativeToken"]["address"].as_str().unwrap_or("");
        let native_token_link = format!("https://etherscan.io/address/{:?}", native_token_address);
        let abbreviated_native_address = format!(
            "{}...{}",
            &native_token_address[..6],
            &native_token_address[native_token_address.len() - 4..]
        );

        Row::new(vec![
            Cell::from(chain["id"].as_i64().unwrap_or(0).to_string()),
            Cell::from(chain["key"].as_str().unwrap_or("")),
            Cell::from(chain["name"].as_str().unwrap_or("")),
            Cell::from(chain["chainType"].as_str().unwrap_or("")),
            Cell::from(chain["coin"].as_str().unwrap_or("")),
            Cell::from(chain["mainnet"].as_bool().unwrap_or(false).to_string()),
            Cell::from("Test"),
            Cell::from(etherscan_link.clone()), // Cell::from("Test".to_string()),
                                                // Cell::from(format!(
                                                //     "\x1B]8;;{}\x07{}\x1B]8;;\x07",
                                                //     etherscan_link, abbreviated_address
                                                // ))
                                                // .style(Style::default().fg(Color::Cyan)),
                                                // Cell::from(format!(
                                                //     "\x1B]8;;{}\x07{}\x1B]8;;\x07",
                                                //     native_token_link, abbreviated_native_address
                                                // ))
                                                // .style(Style::default().fg(Color::Cyan)),
        ])
    });

    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Chains"))
        .widths(&[
            Constraint::Percentage(5),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
}
