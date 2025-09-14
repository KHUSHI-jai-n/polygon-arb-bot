# Polygon Arbitrage Detector Bot

A Rust application that detects potential arbitrage opportunities on the **Polygon network** by comparing token prices across two decentralized exchanges (DEXes) — **QuickSwap** and **SushiSwap**.

Arbitrage in this context means:
- Buying a token cheaply on one DEX
- Immediately selling it for a higher price on another DEX
- Logging the simulated profit opportunity (without executing real trades)

---

## Features
- Connects to Polygon RPC node
- Fetches WETH → USDC price from **QuickSwap** and **SushiSwap**
- Detects price differences and calculates simulated profit
- Configurable:
  - RPC endpoint
  - DEX router addresses
  - Token addresses
  - Trade size, gas cost, profit threshold

---

## Technology Stack
- **Language:** Rust
- **Blockchain:** Polygon Network
- **DEXes:** QuickSwap, SushiSwap (Uniswap V2-style routers)
- **Crates used:**
  - [`ethers-rs`](https://docs.rs/ethers/latest/ethers/) → Ethereum/Polygon contract calls
  - `tokio` → Async runtime
  - `serde`, `toml` → Config parsing
  - `anyhow` → Error handling
