use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt};
use serde_json::Value;
use tracing::{info, warn, error};
use dashmap::DashMap;
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct LiveMatchData {
    pub game_id: u64,
    pub slug: String,
    pub home_team: String,
    pub away_team: String,
    pub score: String,
    pub period: String,
    pub elapsed: String,
    pub status: String,
    pub live: bool,
}

pub struct SportsWebSocket {
    url: String,
    pub cache: Arc<DashMap<String, LiveMatchData>>,
}

impl SportsWebSocket {
    pub fn new() -> Self {
        Self { url: "wss://sports-api.polymarket.com/ws".to_string(), cache: Arc::new(DashMap::new()) }
    }

    pub async fn run(&self) -> Result<()> {
        info!("⚽ Connecting to Sports WebSocket...");
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (_, mut read) = ws_stream.split();
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                if let Ok(data) = serde_json::from_str::<Value>(&text) {
                    self.parse_sport_data(data);
                }
            }
        }
        Ok(())
    }

    fn parse_sport_data(&self, data: Value) {
        if let Some(records) = data.get("records").and_then(|r| r.as_array()) {
            for record in records {
                if let Some(game_id) = record.get("gameId").and_then(|v| v.as_u64()) {
                    let slug = record.get("slug").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let match_data = LiveMatchData {
                        game_id,
                        slug: slug.clone(),
                        home_team: record.get("homeTeam").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
                        away_team: record.get("awayTeam").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
                        score: record.get("score").and_then(|v| v.as_str()).unwrap_or("0-0").to_string(),
                        period: record.get("period").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        elapsed: record.get("elapsed").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
                        status: record.get("status").and_then(|v| v.as_str()).unwrap_or("Scheduled").to_string(),
                        live: record.get("live").and_then(|v| v.as_bool()).unwrap_or(false),
                    };
                    self.cache.insert(slug, match_data);
                }
            }
        }
    }
}
