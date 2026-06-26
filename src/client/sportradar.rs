use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use chrono::Utc;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub id: String,
    pub status: String,
    pub scheduled: String,
    pub home: Team,
    pub away: Team,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub alias: Option<String>,
}

pub struct SportradarClient {
    client: Client,
    api_key: String,
}

impl SportradarClient {
    pub fn new(api_key: String) -> Self {
        Self { client: Client::new(), api_key }
    }

    fn build_url(&self, sport: &str, endpoint: &str) -> String {
        let base = match sport {
            "soccer" => "https://api.sportradar.com/soccer/trial/v4/en",
            "nba" => "https://api.sportradar.com/nba/trial/v7/en",
            "mlb" => "https://api.sportradar.com/mlb/trial/v7/en",
            "global_basketball" => "https://api.sportradar.com/global-basketball/trial/v7/en",
            _ => "https://api.sportradar.com/soccer/trial/v4/en",
        };
        format!("{}{}?api_key={}", base, endpoint, self.api_key)
    }

    pub async fn get_todays_schedule(&self, sport: &str) -> Result<Vec<Game>> {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let url = self.build_url(sport, &format!("/games/{}/schedule", date));
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let data: ScheduleResponse = response.json().await?;
            Ok(data.games)
        } else {
            warn!("Sportradar {} error", sport);
            Ok(Vec::new())
        }
    }

    pub async fn get_live_games(&self, sport: &str) -> Result<Vec<Game>> {
        let games = self.get_todays_schedule(sport).await?;
        let live: Vec<Game> = games.into_iter().filter(|g| g.status == "inprogress" || g.status == "halftime").collect();
        Ok(live)
    }
}
