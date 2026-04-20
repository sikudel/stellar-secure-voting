# ============================================================
# Secure Voting Smart Contract – Makefile
# ============================================================

# Build the contract to WASM
build:
	soroban contract build

# Run unit tests
test:
	cargo test

# Deploy to Stellar testnet  (replace IDENTITY with your funded account name)
deploy:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/voting_contract.wasm \
		--source IDENTITY \
		--network testnet
