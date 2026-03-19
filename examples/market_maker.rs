//! Simple market-making example.
//!
//! Posts symmetric bid/ask quotes around the current mid price
//! and refreshes them periodically.
//!
//! Usage:
//!   export HEX_API_URL=https://api.hexmarket.io
//!   export HEX_PUBKEY=<your-solana-pubkey>
//!   export HEX_API_KEY=<api-key>
//!   export HEX_SECRET=<base64url-secret>
//!   export HEX_PASSPHRASE=<passphrase>
//!   export HEX_KEYPAIR_B58=<base58-encoded-ed25519-keypair-64-bytes>
//!   export HEX_OUTCOME_ID=<outcome-uuid>
//!   cargo run --example market_maker

use hexmarket_sdk::*;
use rust_decimal::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = std::env::var("HEX_API_URL").unwrap_or_else(|_| "http://localhost:8080".into());
    let pubkey = std::env::var("HEX_PUBKEY").expect("HEX_PUBKEY required");
    let api_key = std::env::var("HEX_API_KEY").expect("HEX_API_KEY required");
    let secret = std::env::var("HEX_SECRET").expect("HEX_SECRET required");
    let passphrase = std::env::var("HEX_PASSPHRASE").expect("HEX_PASSPHRASE required");
    let keypair_b58 = std::env::var("HEX_KEYPAIR_B58").expect("HEX_KEYPAIR_B58 required");
    let outcome_id = std::env::var("HEX_OUTCOME_ID").expect("HEX_OUTCOME_ID required");

    // Decode Ed25519 keypair (64 bytes: 32 secret + 32 public)
    let keypair_bytes = bs58::decode(&keypair_b58).into_vec()?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(
        keypair_bytes[..32].try_into().expect("need 32 bytes"),
    );

    let client = HexClient::new(HexClientConfig { api_url });
    client.set_credentials(
        &pubkey,
        ApiCredentials {
            api_key,
            secret,
            passphrase,
        },
    );

    let spread = Decimal::new(2, 2); // 0.02 (2 cents)
    let qty = 5u64;

    loop {
        // 1. Get current orderbook
        let book = client.get_orderbook(&outcome_id).await?;

        let best_bid = book.bids.first().map(|l| l.price);
        let best_ask = book.asks.first().map(|l| l.price);

        let mid = match (best_bid, best_ask) {
            (Some(b), Some(a)) => (b + a) / Decimal::TWO,
            (Some(b), None) => b + spread / Decimal::TWO,
            (None, Some(a)) => a - spread / Decimal::TWO,
            (None, None) => Decimal::new(50, 2), // default 0.50
        };

        let bid_price = (mid - spread / Decimal::TWO).max(Decimal::new(1, 2));
        let ask_price = (mid + spread / Decimal::TWO).min(Decimal::new(99, 2));

        println!(
            "mid={} bid={} ask={} qty={}",
            mid, bid_price, ask_price, qty
        );

        // 2. Cancel existing open orders
        let open_orders = client.get_open_orders(Some(&outcome_id)).await?;
        for order in &open_orders {
            if let Err(e) = client.cancel_order(&order.id.to_string()).await {
                eprintln!("cancel failed: {}", e);
            }
        }

        // 3. Place bid
        let nonce = auth::generate_nonce();
        let msg = auth::build_order_message(
            &outcome_id,
            "buy",
            &bid_price.to_string(),
            qty,
            nonce,
        );
        let sig = auth::ed25519_sign(&signing_key, &msg);

        match client
            .place_order(&PlaceOrderParams {
                outcome_id: outcome_id.clone(),
                side: Side::Buy,
                order_type: OrderType::Limit,
                time_in_force: TimeInForce::Gtc,
                price: bid_price,
                quantity: qty,
                nonce,
                signature: sig,
            })
            .await
        {
            Ok(r) => println!("BID placed: {}", r.order_id),
            Err(e) => eprintln!("BID failed: {}", e),
        }

        // 4. Place ask
        let nonce = auth::generate_nonce();
        let msg = auth::build_order_message(
            &outcome_id,
            "sell",
            &ask_price.to_string(),
            qty,
            nonce,
        );
        let sig = auth::ed25519_sign(&signing_key, &msg);

        match client
            .place_order(&PlaceOrderParams {
                outcome_id: outcome_id.clone(),
                side: Side::Sell,
                order_type: OrderType::Limit,
                time_in_force: TimeInForce::Gtc,
                price: ask_price,
                quantity: qty,
                nonce,
                signature: sig,
            })
            .await
        {
            Ok(r) => println!("ASK placed: {}", r.order_id),
            Err(e) => eprintln!("ASK failed: {}", e),
        }

        // 5. Wait before refreshing
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
