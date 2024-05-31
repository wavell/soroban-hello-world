
# Lora Demo Installation
## General Stella installation instructions
https://developers.stellar.org/docs/smart-contracts/getting-started/setup

## Walkthrough
## Install specific version of cargo cli
```
cargo install --git https://github.com/stellar/stellar-cli soroban-cli
```

```
git clone https://github.com/wavell/lora-demo.git

## probably don't need to do
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

## probably don't need to do	
soroban keys generate --global alice --network testnet
	
cargo build --target wasm32-unknown-unknown --release

soroban keys generate alice --network testnet

## returns the contract id ... save it for later
soroban contract deploy   --wasm target/wasm32-unknown-unknown/release/hello_world.wasm   --source "alice"   --network testnet
                                      
## Useful network commands                   
soroban network ls                                
soroban keys generate alice --network testnet


## Check the contract parameters                                                                                                  
soroban contract invoke --id <<your contract id>> --network testnet -h

## Call the smart contract  
soroban contract invoke --id <<your contract id>> --network testnet --source alice -- store_value --value '[[-42, -42]]' --key hello
soroban contract invoke --id <<your contract id>> --network testnet --source alice -- retrieve_value --key hello
```
## Soroban Cli Wrapper
To run the web server that runs as a wrapper for the soroban client
```
bun run sorobanserver.js
## Curl comnand to test the soroban wrapper
curl -X POST http://localhost:3000/invoke   -H "Content-Type: application/json"   -d '{
    "id": "<<your contract id>>",
    "sourceAccount": "alice",
    "value": "[[-42, -42]]",
    "key": "hello"
  }'

```
## 
## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
