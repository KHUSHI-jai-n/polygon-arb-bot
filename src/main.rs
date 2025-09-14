use ethers::prelude::*;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;

// Uniswap V2 Router ABI
abigen!(
    UniswapV2Router,
    r#"[function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)]"#
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config
    let config: Config = load_config("config.toml")?;
    println!("Loaded config: {:?}", config);

    // Connect to Polygon RPC
    let provider = Provider::<Http>::try_from(config.rpc_url.clone())?;
    let client = Arc::new(provider);

    println!("Connected to Polygon RPC");

    // Check Polygon block
    let block = client.get_block_number().await?;
    println!("Current Polygon block number: {}", block);

    // Amount in (1 WETH)
    let amount_in = U256::exp10(18); 

    // Path: WETH → USDC
    let path: Vec<Address> = vec![
        config.weth.parse::<Address>()?,
        config.usdc.parse::<Address>()?,
    ];

    // Fetch prices from both DEXes
    let price_quick = get_price(&client, &config.quickswap_router, amount_in, &path).await?;
    let price_sushi = get_price(&client, &config.sushiswap_router, amount_in, &path).await?;

    println!("QuickSwap: 1 WETH = {} USDC", price_quick);
    println!("SushiSwap: 1 WETH = {} USDC", price_sushi);

    // Arbitrage Detection
    let profit = simulate_profit(price_quick, price_sushi, config.trade_size, config.gas_cost);

    if profit > config.min_profit {
        println!("Arbitrage Opportunity! Profit: {:.2} USDC", profit);
    } else {
        println!("No arbitrage opportunity yet. (Profit = {:.2} USDC)", profit);
    }

    Ok(())
}

/// Get price from a DEX router
async fn get_price(
    client: &Arc<Provider<Http>>,
    router_addr: &str,
    amount_in: U256,
    path: &Vec<Address>,
) -> anyhow::Result<f64> {
    let router = UniswapV2Router::new(router_addr.parse::<Address>()?, client.clone());
    let amounts = router.get_amounts_out(amount_in, path.clone()).call().await?;
    let usdc_out = amounts[1];
    Ok(usdc_out.as_u128() as f64 / 1e6) // USDC has 6 decimals
}

/// Config structure
#[derive(Debug, Deserialize)]
struct Config {
    rpc_url: String,
    quickswap_router: String,
    sushiswap_router: String,
    weth: String,
    usdc: String,
    trade_size: f64,
    min_profit: f64,
    gas_cost: f64,
}

/// Load config file
fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Simulated profit calculation
fn simulate_profit(price_a: f64, price_b: f64, trade_size: f64, gas_cost: f64) -> f64 {
    let buy_price = price_a.min(price_b);
    let sell_price = price_a.max(price_b);

    let revenue = sell_price * trade_size;
    let cost = buy_price * trade_size + gas_cost;

    revenue - cost
}
