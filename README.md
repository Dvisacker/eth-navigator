# eth-manager - EVM Chain Manager Tool

eth-manager is a command-line tool for interacting with Ethereum and Ethereum-compatible blockchains. It provides various functionalities such as querying blockchain data, subscribing to events sending/bridging assets, generating contract bindings, abi or source code. To avoid errors, tokens and wallets must be whitelisted. 

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Run the following command to build and install eth-manager:

```
make install
```

Make sure `/usr/local/bin` is in your PATH.

## Usage

To use eth-manager, run the following command:

```
eth-manager <COMMAND> [OPTIONS]
```

Replace `<COMMAND>` with one of the available commands listed below, and `[OPTIONS]` with the appropriate options for that command.

## Available Commands and Examples

### EVM Interface Commands

1. Get Block Number
```
eth-manager get-block-number --network ethereum
```

2. Subscribe to Blocks
```
eth-manager subscribe-blocks --network ethereum
```

3. Subscribe to Pending Transactions
```
eth-manager subscribe-pending-transactions --network ethereum
```

4. Get Gas Price
```
eth-manager get-gas-price --network ethereum
```

5. Get Balance
```
eth-manager get-balance --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

6. Get Nonce
```
eth-manager get-nonce --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

7. Get Block Details
```
eth-manager get-block-details --block-number 12345678 --network ethereum
```

8. Subscribe to Logs
```
eth-manager subscribe-logs --network ethereum
```

9. Get Transaction Details
```
eth-manager get-tx-details --tx-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef --network ethereum
```

10. Generate Contract Bindings
```
eth-manager generate-contract-bindings --contract-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --contract-name DAI --network ethereum
```

11. Generate Source Code
```
eth-manager generate-source-code --contract-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --contract-name DAI --network ethereum
```

12. Get ERC20 Balance
```
eth-manager get-erc20-balance --wallet-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --network ethereum
```

13. Wrap ETH
```
eth-manager wrap-eth --amount 1000000000000000000 --network ethereum
```

14. Send ETH
```
eth-manager send-eth --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --amount 1000000000000000000 --network ethereum
```

15. Send ERC20
```
eth-manager send-erc20 --token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --amount 1000000000000000000 --network ethereum
```

16. Get Transactions
```
eth-manager get-transactions --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

17. Swap Tokens on Uniswap V3
```
eth-manager swap-tokens-uniswap-v3 --token-in 0x6B175474E89094C44Da98b954EedeAC495271d0F --token-out 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 --amount-in 1000000000000000000 --amount-out-minimum 1000000000000000000 --recipient 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --network ethereum
```

### LI.FI Bridge Commands

18. Get Supported Chains
```
eth-manager get-supported-chains
```

19. Get Known Tokens
```
eth-manager get-known-tokens --chain ethereum
```

20. Request Routes
```
eth-manager request-routes --from-chain-id 1 --to-chain-id 137 --from-token-address 0x6B175474E89094C44Da98b954EedeAC495271d0F --to-token-address 0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063 --from-amount 1000000000000000000
```

21. Request Quote
```
eth-manager request-quote --from-chain ethereum --to-chain polygon --from-token DAI --to-token DAI --from-amount 1000000000000000000 --from-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --to-address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

22. Get Transfer Status
```
eth-manager get-transfer-status --tx-hash 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef --from-chain ethereum --to-chain polygon
```

23. Get Connections
```
eth-manager get-connections --from-chain ethereum --to-chain polygon --from-token DAI --to-token MATIC --from-amount 1000000000000000000 --allow-exchanges true
```

### Whitelist Management Commands

24. Add Wallet to Whitelist
```
eth-manager add-wallet-to-whitelist --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e --name "My Wallet"
```

25. Remove Wallet from Whitelist
```
eth-manager remove-wallet-from-whitelist --address 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
```

26. Add Token to Whitelist
```
eth-manager add-token-to-whitelist --address 0x6B175474E89094C44Da98b954EedeAC495271d0F --name "DAI" --chain ethereum
```

27. Remove Token from Whitelist
```
eth-manager remove-token-from-whitelist --address 0x6B175474E89094C44Da98b954EedeAC495271d0F --chain ethereum
```

28. Show Whitelist
```
eth-manager show-whitelist
```

## Roadmap

- [x] Basic EVM interactions (get balance, send transactions, etc.)
- [x] ERC20 Send Tokens
- [x] ERC20 Get Balance
- [x] Block and transaction monitoring
- [x] Contract binding generation
- [x] Whitelist management for wallets and tokens
- [ ] Aave Supply/Borrow
- [ ] Uniswap Swap
- [ ] Uniswap Provide Liquidity
- [ ] Lifi integration
- [ ] Aave Flash Loan
- [ ] CoWSwap Limit Order
- [ ] Improve error handling
- [ ] Interactive confirmation when sending transaction
- [ ] Ledger support
- [ ] Debug tranasction list
- [ ] Add tests

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