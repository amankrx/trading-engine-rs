// src/models/trading/position.rs

use serde::{Deserialize, Serialize};
use serde_json::error::Result as JsonResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub qty: String,
    pub avg_entry_price: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub current_price: String,
    pub asset_marginable: bool,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Position {{ symbol: {}, side: {}, qty: {}, avg_entry_price: {}, unrealized_pl: {}, unrealized_plpc: {}, current_price: {}, asset_marginable: {} }}",
            self.symbol, self.side, self.qty, self.avg_entry_price, self.unrealized_pl, self.unrealized_plpc, self.current_price, self.asset_marginable
        )
    }
}

impl Position {
    pub fn from_json(json: &str) -> JsonResult<Position> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
