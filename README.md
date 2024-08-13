# Kleptoman - Ethereum Blockchain Interaction Tool

Kleptoman is a command-line tool for interacting with Ethereum and Ethereum-compatible blockchains. It provides various functionalities such as querying blockchain data, subscribing to events, and generating contract bindings.

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

## Available Commands

### 1. Get Block Number

Retrieves the current block number for a specified network.

```
cargo run -- get-block-number --network <NETWORK>
```

### 2. Subscribe to Events

Subscribes to events for a specified contract and network.

```
cargo run -- subscribe-to-events --contract-address <CONTRACT_ADDRESS> --network <NETWORK>
```

### 3. Download Contract and Generate Bindings

Downloads the contract ABI and source code for a specified contract address and network, and generates Rust bindings using Abigen.

```
cargo run -- download-contract-and-generate-bindings --contract-address <CONTRACT_ADDRESS> --contract-name <CONTRACT_NAME> --network <NETWORK>
```

Replace `<CONTRACT_ADDRESS>` with the address of the contract you want to download and generate bindings for, `<CONTRACT_NAME>` with the name of the contract, and `<NETWORK>` with the network the contract is deployed on (e.g., "ethereum", "polygon", etc.).

## Example Usage

```
cargo run -- download-contract-and-generate-bindings --contract-address 0x1234567890abcdef --contract-name MyContract --network ethereum
```

This command will download the ABI and source code for the contract at address `0x1234567890abcdef` on the Ethereum network, and generate Rust bindings for the contract with the name `MyContract`.