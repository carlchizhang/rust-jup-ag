use jup_ag::{QuoteConfig, SwapRequest};
use solana_sdk::{pubkey, signature::Keypair, signature::Signer};
use spl_token::{amount_to_ui_amount, ui_amount_to_amount};

use tracing::{Level, event, span};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let span = span!(Level::DEBUG, "main");
    let _guard = span.enter();
    
    event!(Level::INFO, "Start");

    let sol = pubkey!("So11111111111111111111111111111111111111112");
    let msol = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");

    let keypair = Keypair::new();

    let slippage_bps = 100;
    let only_direct_routes = false;
    event!(Level::INFO, "Quote 1 request");
    let quotes = jup_ag::quote(
        sol,
        msol,
        ui_amount_to_amount(0.01, 9),
        QuoteConfig {
            only_direct_routes,
            slippage_bps: Some(slippage_bps),
            ..QuoteConfig::default()
        },
    )
    .await?;
    event!(Level::INFO, "Quote 1 response");

    let route = quotes.route_plan[0]
        .swap_info
        .label
        .clone()
        .unwrap_or_else(|| "Unknown DEX".to_string());
    println!(
        "Quote: {} SOL for {} mSOL via {} (worst case with slippage: {}). Impact: {:.2}%",
        amount_to_ui_amount(quotes.in_amount, 9),
        amount_to_ui_amount(quotes.out_amount, 9),
        route,
        amount_to_ui_amount(quotes.other_amount_threshold, 9),
        quotes.price_impact_pct * 100.
    );

    event!(Level::INFO, "Swap instru request");
    let request: SwapRequest = SwapRequest::new(keypair.pubkey(), quotes.clone());

    let swap_instructions = jup_ag::swap_instructions(request).await?;
    event!(Level::INFO, "Swap instru response");

    // println!("Swap Instructions: {:?}", swap_instructions);
    event!(Level::INFO, "Done");

    Ok(())
}
