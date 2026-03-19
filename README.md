# HexMarket Rust SDK

Rust client for the [HexMarket](https://hexmarket.io) prediction market API. Built for quantitative trading and market making.

## Installation

```toml
# Cargo.toml
[dependencies]
hexmarket-sdk = { git = "https://github.com/hexmarketxyz/hexmarket_rust_sdk.git" }
```

## Quick Start

```rust
use hexmarket_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HexClient::new(HexClientConfig {
        api_url: "https://api.hexmarket.io".into(),
    });

    // Browse markets (no auth needed)
    let events = client.list_events(&Default::default()).await?;
    for event in &events {
        println!("{}: {} outcomes", event.event.title, event.outcomes.len());
    }

    // Read orderbook
    let outcome_id = &events[0].outcomes[0].id.to_string();
    let book = client.get_orderbook(outcome_id).await?;
    println!("Best bid: {:?}, Best ask: {:?}", book.bids.first(), book.asks.first());

    // Authenticate for trading
    client.set_credentials(
        "YourSolanaPubkey",
        ApiCredentials {
            api_key: "your-api-key".into(),
            secret: "your-base64url-secret".into(),
            passphrase: "your-passphrase".into(),
        },
    );

    let balance = client.get_balance().await?;
    println!("USDC: {} (locked: {})", balance.usdc_balance, balance.locked_usdc);

    Ok(())
}
```

## Authentication

HexMarket uses two authentication layers:

| Layer | Mechanism | Used For |
|-------|-----------|----------|
| **L1** | Ed25519 wallet signature | API key creation |
| **L2** | HMAC-SHA256 with API credentials | All trading endpoints |

For trading, you need API credentials (`api_key`, `secret`, `passphrase`). Create them once via the HexMarket web UI, then pass to `client.set_credentials()` — the SDK handles HMAC signing automatically.

## Placing Orders

Each order requires an Ed25519 wallet signature for on-chain settlement:

```rust
use hexmarket_sdk::*;
use rust_decimal_macros::dec;

let keypair_bytes = bs58::decode("YourBase58Keypair").into_vec()?;
let signing_key = ed25519_dalek::SigningKey::from_bytes(
    keypair_bytes[..32].try_into()?,
);

let nonce = auth::generate_nonce();
let msg = auth::build_order_message("outcome-uuid", "buy", "0.55", 10, nonce);
let sig = auth::ed25519_sign(&signing_key, &msg);

let resp = client.place_order(&PlaceOrderParams {
    outcome_id: "outcome-uuid".into(),
    side: Side::Buy,
    order_type: OrderType::Limit,
    time_in_force: TimeInForce::Gtc,
    price: dec!(0.55),
    quantity: 10,
    nonce,
    signature: sig,
}).await?;

println!("Order placed: {}", resp.order_id);
```

## API Reference

### Public Endpoints (no auth)

| Method | Description |
|--------|-------------|
| `list_events(params)` | List prediction events |
| `get_event(slug)` | Get event detail by slug |
| `list_markets(params)` | List outcomes |
| `get_market(outcome_id)` | Get single outcome |
| `get_orderbook(outcome_id)` | Get orderbook (bids/asks) |
| `get_merged_orderbook(outcome_id)` | Orderbook with cross-outcome liquidity |
| `list_trades(params)` | List trades |
| `list_tags()` | List categories |
| `get_tag(slug)` | Get category with children |

### Authenticated Endpoints (L2 auth)

| Method | Description |
|--------|-------------|
| `place_order(params)` | Place a limit/market order |
| `cancel_order(order_id)` | Cancel an open order |
| `get_open_orders(outcome_id?)` | List open orders |
| `get_closed_orders(outcome_id?)` | List filled/cancelled orders |
| `get_balance()` | USDC balance and locked amount |
| `get_positions()` | Outcome token positions |
| `get_vault_balance()` | On-chain vault USDC balance |
| `create_vault()` | Create user vault |
| `deposit(amount)` | Build deposit transaction |
| `withdraw(amount)` | Build withdrawal transaction |
| `submit_transaction(tx_b64)` | Submit signed Solana transaction |

## Examples

- [`examples/basic.rs`](examples/basic.rs) — Browse events, check balance, read orderbook
- [`examples/market_maker.rs`](examples/market_maker.rs) — Symmetric bid/ask quoting loop

Run examples:

```bash
export HEX_API_URL=https://api.hexmarket.io
cargo run --example basic
```

## Dependencies

- [reqwest](https://docs.rs/reqwest) — HTTP client
- [serde](https://serde.rs/) — Serialization
- [rust_decimal](https://docs.rs/rust_decimal) — Precise decimal arithmetic
- [ed25519-dalek](https://docs.rs/ed25519-dalek) — Ed25519 signing
- [hmac](https://docs.rs/hmac) + [sha2](https://docs.rs/sha2) — HMAC-SHA256

## License

MIT
