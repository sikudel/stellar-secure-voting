# Secure Voting Smart Contract on Stellar 🗳️

A decentralized voting smart contract built using **Soroban** on the **Stellar blockchain**. Backend-only — no frontend required.

---

## Features

| Feature | Details |
|---|---|
| **One wallet = one vote** | Each wallet address can only vote once, enforced cryptographically |
| **Immutable voting data** | All votes are stored on-chain and cannot be altered |
| **Transparent vote counting** | Anyone can query the contract to see live rankings |
| **Permissionless candidates** | Anyone can call `add_candidate` to register a new participant |

---

## Project Structure

```
contracts/voting/
├── Cargo.toml        # Rust dependencies
├── Makefile          # Build & deploy shortcuts
└── src/
    └── lib.rs        # Smart contract logic
```

---

## Prerequisites

1. **Rust** with the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Stellar CLI** (formerly Soroban CLI):
   ```bash
   cargo install --locked stellar-cli --features opt
   ```

3. A **funded testnet account** (identity):
   ```bash
   stellar keys generate --global my-wallet --network testnet --fund
   ```

---

## How to Build

Inside `contracts/voting/`, run:

```bash
stellar contract build
```

The compiled WASM file will be at:
```
target/wasm32-unknown-unknown/release/voting_contract.wasm
```

---

## How to Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/voting_contract.wasm \
  --source my-wallet \
  --network testnet
```

> Copy the **Contract ID** printed in the terminal — you'll need it for all invocations below.

---

## Smart Contract

- **Testnet Contract ID:** `(replace with your generated Contract ID)`
- **Network:** Stellar Testnet
- **Language:** Rust / Soroban SDK v21

---

## Example Usage (Stellar CLI)

### Add a candidate
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-wallet \
  --network testnet \
  -- add_candidate \
  --name "Alice"
```

### Vote for a candidate (candidate_id = 0)
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-wallet \
  --network testnet \
  -- vote \
  --voter <VOTER_ADDRESS> \
  --candidate_id 0
```

### Get all candidates and vote counts
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-wallet \
  --network testnet \
  -- get_candidates
```

### Check if an address has voted
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-wallet \
  --network testnet \
  -- has_voted \
  --voter <VOTER_ADDRESS>
```

---

## Contract Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `add_candidate` | `name: String` | `u32` | Registers a new candidate, returns their ID |
| `vote` | `voter: Address`, `candidate_id: u32` | `void` | Casts a vote, enforces one-vote-per-wallet |
| `get_candidates` | — | `Vec<Candidate>` | Returns all candidates with vote counts |
| `has_voted` | `voter: Address` | `bool` | Returns true if address has already voted |

---

## Repository Name Suggestions

- `stellar-secure-voting`
- `soroban-voting-contract`
- `web3-voting-soroban`

---

## License

MIT
