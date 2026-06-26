use serde::Deserialize;
use reqwest::Client;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct SportsDataIOFixture {
    pub GameID: u32,
    pub HomeTeam: String,
    pub AwayTeam: String,
    pub Status: String,
    pub HomeScore: Option<u32>,
    pub AwayScore: Option<u32>,
    pub DateTime: String,
}

pub struct SportsDataIOClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl SportsDataIOClient {
    pub fn new(api_key: String) -> Self {
        Self { client: Client::new(), api_key, base_url: "https://api.sportsdata.io/v4".to_string() }
    }

    pub async fn get_live_scores(&self, league: &str) -> Result<Vec<SportsDataIOFixture>> {
        let url = format!("{}/soccer/scores/json/LiveScores/{}?key={}", self.base_url, league, self.api_key);
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;
        if let Some(fixtures) = data.as_array() {
            let parsed: Vec<SportsDataIOFixture> = serde_json::from_value(serde_json::Value::Array(fixtures.clone()))?;
            return Ok(parsed);
        }
        Ok(Vec::new())
    }
}
