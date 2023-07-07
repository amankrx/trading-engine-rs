// src/models/trading/account.rs

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::error::Result as JsonResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub status: String,
    pub cash: String,
    pub portfolio_value: String,
    pub buying_power: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Account {{ id: {}, status: {}, cash: {}, portfolio_value: {}, buying_power: {} }}",
            self.id, self.status, self.cash, self.portfolio_value, self.buying_power
        )
    }
}

impl Account {
    pub fn from_json(json: &str) -> JsonResult<Account> {
        serde_json::from_str(json)
    }
}
