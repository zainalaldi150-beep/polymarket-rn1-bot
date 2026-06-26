use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::client::OrderSide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperAccount {
    pub balance: Decimal,
    pub positions: HashMap<String, PaperPosition>,
    pub trade_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPosition {
    pub market_id: String,
    pub token_id: String,
    pub side: String,
    pub shares: Decimal,
    pub avg_price: Decimal,
    pub entry_value: Decimal,
}

pub struct PaperExecutor {
    account: PaperAccount,
    state_file: String,
}

impl PaperExecutor {
    pub fn new(initial_balance: Decimal, state_file: &str) -> Self {
        Self { account: PaperAccount { balance: initial_balance, positions: HashMap::new(), trade_count: 0 }, state_file: state_file.to_string() }
    }

    pub fn load_state(&mut self) -> Result<(), anyhow::Error> {
        if let Ok(contents) = std::fs::read_to_string(&self.state_file) {
            if let Ok(account) = serde_json::from_str(&contents) { self.account = account; }
        }
        Ok(())
    }

    fn save_state(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.account) {
            let _ = std::fs::write(&self.state_file, json);
        }
    }

    pub fn buy(&mut self, market_id: &str, token_id: &str, side: OrderSide, price: Decimal, shares: Decimal) -> Result<(), String> {
        let cost = price * shares;
        if cost > self.account.balance { return Err(format!("Saldo tidak cukup: butuh ${:.2}, tersisa ${:.2}", cost, self.account.balance)); }
        self.account.balance -= cost;
        let side_str = match side { OrderSide::Buy => "YES", OrderSide::Sell => "NO" };
        let key = format!("{}_{}_{}", market_id, token_id, side_str);
        if let Some(pos) = self.account.positions.get_mut(&key) {
            let total_cost = pos.avg_price * pos.shares + cost;
            pos.shares += shares;
            pos.avg_price = total_cost / pos.shares;
            pos.entry_value = pos.avg_price * pos.shares;
        } else {
            self.account.positions.insert(key, PaperPosition { market_id: market_id.to_string(), token_id: token_id.to_string(), side: side_str.to_string(), shares, avg_price: price, entry_value: cost });
        }
        self.account.trade_count += 1;
        info!("📝 [PAPER] BUY {} | {} shares @ {:.2}¢", market_id, shares, price * Decimal::from(100));
        self.save_state();
        Ok(())
    }

    pub fn sell(&mut self, market_id: &str, token_id: &str, side: OrderSide, price: Decimal, shares: Decimal) -> Result<(), String> {
        let side_str = match side { OrderSide::Buy => "YES", OrderSide::Sell => "NO" };
        let key = format!("{}_{}_{}", market_id, token_id, side_str);
        let pos = self.account.positions.get_mut(&key).ok_or("Posisi tidak ditemukan")?;
        if pos.shares < shares { return Err("Shares tidak mencukupi".to_string()); }
        pos.shares -= shares;
        let revenue = price * shares;
        self.account.balance += revenue;
        if pos.shares == Decimal::ZERO { self.account.positions.remove(&key); } else { pos.entry_value = pos.avg_price * pos.shares; }
        self.save_state();
        Ok(())
    }

    pub fn get_balance(&self) -> Decimal { self.account.balance }
}
