use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use rust_decimal::Decimal;
use tracing::{info, error};
use anyhow::Result;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct PriceUpdate {
    pub market_id: String,
    pub token_id: String,
    pub price: Decimal,
}

pub struct WebSocketManager {
    config: Arc<Config>,
    price_tx: mpsc::Sender<PriceUpdate>,
}

impl WebSocketManager {
    pub fn new(config: Arc<Config>, price_tx: mpsc::Sender<PriceUpdate>) -> Self {
        Self { config, price_tx }
    }

    pub async fn run(&self) -> Result<()> {
        let url = self.config.polymarket.ws_url.clone();
        info!("🔌 Connecting to WebSocket: {}", url);
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        let subscribe_msg = json!({ "type": "subscribe", "channels": ["market"] }).to_string();
        write.send(Message::Text(subscribe_msg)).await?;
        info!("✅ Subscribed to market channel");

        let price_tx = self.price_tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(price_data) = data.get("price").or(data.get("data")) {
                            if let Some(market_id) = price_data.get("market").and_then(|v| v.as_str()) {
                                if let Some(token_id) = price_data.get("token_id").and_then(|v| v.as_str()) {
                                    if let Some(price_str) = price_data.get("price").and_then(|v| v.as_str()) {
                                        if let Ok(price) = Decimal::from_str(price_str) {
                                            let _ = price_tx.send(PriceUpdate { market_id: market_id.to_string(), token_id: token_id.to_string(), price }).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}
