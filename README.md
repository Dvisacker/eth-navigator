# Kleptoman - Ethereum Blockchain Interaction Tool

Kleptoman is a command-line tool for interacting with Ethereum and Ethereum-compatible blockchains. It provides various functionalities such as querying blockchain data, subscribing to events, generating contract bindings, and interacting with the LI.FI bridge.

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Run `cargo build --release` in the project directory.

## Usage

To use Kleptoman, run the following command:

```
cargo run -- <COMMAND> [OPTIONS]
```

Replace `<COMMAND>` with one of the available commands listed below, and `[OPTIONS]` with the appropriate options for that command.

## Available Commands and Examples

### EVM Interface Commands

1. Get Block Number
```
cargo run -- get-block-number --network ethereum
```

2. Subscribe to Blocks
```
cargo run -- subscribe-blocks --network ethereum
```

3. Subscribe to Pending Transactions
```
cargo run -- subscribe-pending-transactions --network ethereum
```

4. Get Gas Price
```
cargo run -- get-gas-price --network ethereum
```

5. Get Balance
```
cargo run -- get-balance --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

6. Get Nonce
```
cargo run -- get-nonce --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

7. Get Block Details
```
cargo run -- get-block-details --block-number 12345678 --network ethereum
```

8. Subscribe to Logs
```
cargo run -- subscribe-logs --network ethereum
```

9. Get Transaction Details
```
cargo run -- get-tx-details --tx-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef --network ethereum
```

10. Generate Contract Bindings
```
cargo run -- generate-contract-bindings --contract-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --contract-name DAI --network ethereum
```

11. Generate Source Code
```
cargo run -- generate-source-code --contract-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --contract-name DAI --network ethereum
```

12. Get ERC20 Balance
```
cargo run -- get-erc20-balance --wallet-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --network ethereum
```

13. Wrap ETH
```
cargo run -- wrap-eth --amount 1000000000000000000 --network ethereum
```

14. Send ETH
```
cargo run -- send-eth --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --amount 1000000000000000000 --network ethereum
```

15. Send ERC20
```
cargo run -- send-erc20 --token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --amount 1000000000000000000 --network ethereum
```

### LI.FI Bridge Commands

16. Get Supported Chains
```
cargo run -- get-supported-chains
```

17. Get Known Tokens
```
cargo run -- get-known-tokens --chain ethereum
```

18. Request Routes
```
cargo run -- request-routes --from-chain-id 1 --to-chain-id 137 --from-token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --to-token-address 0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063 --from-amount 1000000000000000000 --from-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

19. Request Quote
```
cargo run -- request-quote --from-chain ethereum --to-chain polygon --from-token DAI --to-token DAI --from-amount 1000000000000000000 --from-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

20. Get Transfer Status
```
cargo run -- get-transfer-status --tx-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef --from-chain ethereum --to-chain polygon
```

21. Get Connections

Retrieves and filters possible connections between chains and tokens from the LI.FI API.

```
cargo run -- get-connections [--from-chain <FROM_CHAIN>] [--to-chain <TO_CHAIN>] [--from-token <FROM_TOKEN>] [--to-token <TO_TOKEN>] [--from-amount <FROM_AMOUNT>] [--allow-exchanges <ALLOW_EXCHANGES>]
```

## Notes

- Replace placeholder addresses, transaction hashes, and other values with actual data when using these commands.
- Some commands may require additional setup, such as setting environment variables for API keys or wallet private keys.
- The `--network` option defaults to "ethereum" for most commands if not specified.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.