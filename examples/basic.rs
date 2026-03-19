//! Basic SDK usage: browse events, check balance, read orderbook.
//!
//! Usage:
//!   export HEX_API_URL=https://api.hexmarket.xyz
//!   cargo run --example basic
//!
//! For authenticated endpoints, also set:
//!   export HEX_PUBKEY=<your-solana-pubkey>
//!   export HEX_API_KEY=<api-key>
//!   export HEX_SECRET=<base64url-secret>
//!   export HEX_PASSPHRASE=<passphrase>

use hexmarket_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = std::env::var("HEX_API_URL").unwrap_or_else(|_| "http://localhost:8080".into());

    let client = HexClient::new(HexClientConfig { api_url });

    // --- Public endpoints (no auth) ---

    // List active events
    let events = client
        .list_events(&api::ListEventsParams {
            status: Some("active".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    println!("=== Active Events ===");
    for item in &events {
        println!(
            "  {} ({} outcomes)",
            item.event.title,
            item.outcomes.len()
        );
        for outcome in &item.outcomes {
            let price_str = outcome
                .price
                .map(|p| format!("{}c", (p * rust_decimal::Decimal::from(100)).round()))
                .unwrap_or_else(|| "n/a".into());
            println!("    - {} @ {}", outcome.label, price_str);
        }
    }

    // Get orderbook for first outcome (if any)
    if let Some(first_outcome) = events.first().and_then(|e| e.outcomes.first()) {
        let book = client.get_orderbook(&first_outcome.id.to_string()).await?;
        println!("\n=== Orderbook: {} ===", first_outcome.label);
        println!("  Bids:");
        for level in book.bids.iter().take(5) {
            println!("    {} @ {}", level.quantity, level.price);
        }
        println!("  Asks:");
        for level in book.asks.iter().take(5) {
            println!("    {} @ {}", level.quantity, level.price);
        }
    }

    // Recent trades
    let trades = client
        .list_trades(&api::ListTradesParams {
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    println!("\n=== Recent Trades ===");
    for trade in &trades {
        println!(
            "  {} {} @ {} (qty {})",
            trade.side, trade.outcome_id, trade.price, trade.quantity
        );
    }

    // --- Authenticated endpoints ---

    let pubkey = std::env::var("HEX_PUBKEY").ok();
    let api_key = std::env::var("HEX_API_KEY").ok();
    let secret = std::env::var("HEX_SECRET").ok();
    let passphrase = std::env::var("HEX_PASSPHRASE").ok();

    if let (Some(pk), Some(ak), Some(s), Some(pp)) = (pubkey, api_key, secret, passphrase) {
        client.set_credentials(&pk, ApiCredentials {
            api_key: ak,
            secret: s,
            passphrase: pp,
        });

        let balance = client.get_balance().await?;
        println!("\n=== Balance ===");
        println!(
            "  USDC: {} (locked: {})",
            balance.usdc_balance, balance.locked_usdc
        );

        let positions = client.get_positions().await?;
        println!("\n=== Positions ===");
        for pos in &positions {
            println!(
                "  outcome={} qty={} avg_price={:?}",
                pos.outcome_id, pos.quantity, pos.avg_price
            );
        }

        let open = client.get_open_orders(None).await?;
        println!("\n=== Open Orders ({}) ===", open.len());
        for order in open.iter().take(10) {
            println!(
                "  {} {} {} @ {} qty={}/{}",
                order.id, order.side, order.outcome_id, order.price,
                order.filled_quantity, order.quantity
            );
        }
    } else {
        println!("\nSkipping authenticated endpoints (set HEX_PUBKEY, HEX_API_KEY, HEX_SECRET, HEX_PASSPHRASE)");
    }

    Ok(())
}
