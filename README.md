# Kleptoman - Ethereum Blockchain Interaction Tool

Kleptoman is a command-line tool for interacting with Ethereum and Ethereum-compatible blockchains. It provides various functionalities such as querying blockchain data, subscribing to events, generating contract bindings, interacting with the LI.FI bridge, and managing whitelists.

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

16. Get Transactions
```
cargo run -- get-transactions --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

### LI.FI Bridge Commands

17. Get Supported Chains
```
cargo run -- get-supported-chains
```

18. Get Known Tokens
```
cargo run -- get-known-tokens --chain ethereum
```

19. Request Routes
```
cargo run -- request-routes --from-chain-id 1 --to-chain-id 137 --from-token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --to-token-address 0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063 --from-amount 1000000000000000000
```

20. Request Quote
```
cargo run -- request-quote --from-chain ethereum --to-chain polygon --from-token DAI --to-token DAI --from-amount 1000000000000000000 --from-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

21. Get Transfer Status
```
cargo run -- get-transfer-status --tx-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef --from-chain ethereum --to-chain polygon
```

22. Get Connections
```
cargo run -- get-connections --from-chain ethereum --to-chain polygon --from-token DAI --to-token MATIC --from-amount 1000000000000000000 --allow-exchanges true
```

### Whitelist Management Commands

23. Add Wallet to Whitelist
```
cargo run -- add-wallet-to-whitelist --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --name "My Wallet"
```

24. Remove Wallet from Whitelist
```
cargo run -- remove-wallet-from-whitelist --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

25. Add Token to Whitelist
```
cargo run -- add-token-to-whitelist --address 0x6B175474E89094C44Da98b954EedeAC495271d0F --name "DAI" --chain ethereum
```

26. Remove Token from Whitelist
```
cargo run -- remove-token-from-whitelist --address 0x6B175474E89094C44Da98b954EedeAC495271d0F --chain ethereum
```

27. Show Whitelist
```
cargo run -- show-whitelist
```

## Roadmap

- [x] Basic EVM interactions (get balance, send transactions, etc.)
- [x] Contract interaction (ERC20, WETH)
- [x] Block and transaction monitoring
- [x] Contract binding generation
- [x] LI.FI integration for cross-chain operations
- [x] Whitelist management for wallets and tokens
- [ ] Supply assets on Aave
- [ ] Swap tokens on Uniswap
- [ ] Get a flash loan
- [ ] Set a limit order on CoWSwap
- [ ] Integrate Pendle functionality
- [ ] Bridge assets with LI.FI or Jumper
- [ ] Implement Executor functionality for automated operations
- [ ] Improve error handling and user feedback
- [ ] Add comprehensive testing suite
- [ ] Create user-friendly documentation

This roadmap is subject to change as the project evolves. Contributions and suggestions for new features are always welcome!

## Notes

- Replace placeholder addresses, transaction hashes, and other values with actual data when using these commands.
- Some commands may require additional setup, such as setting environment variables for API keys or wallet private keys.
- The `--network` option defaults to "ethereum" for most commands if not specified.
- The whitelist is stored in a file named `whitelist.json` in the project root directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.