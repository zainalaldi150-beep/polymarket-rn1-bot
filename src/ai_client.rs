use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl AIClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        }
    }

    pub async fn predict_win_probability(
        &self,
        home_team: &str,
        away_team: &str,
        score: &str,
        minute: u32,
        market_price: f64,
    ) -> Result<f64> {
        let prompt = format!(
            "Anda adalah analis sepak bola profesional. 
            Pertandingan: {} vs {}.
            Skor saat ini: {}, menit ke-{}.
            Harga pasar Polymarket untuk kemenangan {} adalah {:.2}%.
            
            Berdasarkan analisis Anda, berikan probabilitas sebenarnya 
            untuk kemenangan {}.
            
            Jawab HANYA dengan angka desimal antara 0.00 dan 1.00. 
            Contoh: 0.45",
            home_team, away_team, score, minute, 
            home_team, market_price * 100.0,
            home_team
        );

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "Kamu adalah analis sepak bola profesional yang memberikan prediksi akurat berdasarkan data statistik dan kondisi pertandingan.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.2,
            max_tokens: Some(20),
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let data: ChatResponse = response.json().await?;
            if let Some(choice) = data.choices.first() {
                let content = choice.message.content.trim();
                if let Ok(prob) = content.parse::<f64>() {
                    return Ok(prob.clamp(0.0, 1.0));
                }
                for word in content.split_whitespace() {
                    if let Ok(prob) = word.parse::<f64>() {
                        return Ok(prob.clamp(0.0, 1.0));
                    }
                }
                let re = regex::Regex::new(r"(\d+\.\d+)").unwrap();
                if let Some(cap) = re.captures(content) {
                    if let Ok(prob) = cap[1].parse::<f64>() {
                        return Ok(prob.clamp(0.0, 1.0));
                    }
                }
            }
        }

        Ok(market_price)
    }

    pub async fn analyze_market(
        &self,
        home_team: &str,
        away_team: &str,
        score: &str,
        minute: u32,
        market_price: f64,
    ) -> Result<(f64, String)> {
        let prob = self.predict_win_probability(home_team, away_team, score, minute, market_price).await?;
        let diff = prob - market_price;
        let recommendation = if diff > 0.05 {
            format!("BELI (EV +{:.1}%)", diff * 100.0)
        } else if diff < -0.05 {
            format!("LEWATI (EV {:.1}%)", diff * 100.0)
        } else {
            "NETRAL".to_string()
        };
        Ok((prob, recommendation))
    }
}
