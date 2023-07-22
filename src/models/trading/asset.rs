// src/models/trading/asset.rs

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::error::Result as JsonResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub id: String,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
}

impl Display for Asset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Asset Information:")?;
        writeln!(f, "  ID: {}", self.id)?;
        writeln!(f, "  Class: {}", self.class)?;
        writeln!(f, "  Exchange: {}", self.exchange)?;
        writeln!(f, "  Symbol: {}", self.symbol)?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Status: {}", self.status)?;
        writeln!(f, "  Tradable: {}", self.tradable)?;
        writeln!(f, "  Marginable: {}", self.marginable)?;
        writeln!(f, "  Shortable: {}", self.shortable)?;
        writeln!(f, "  Easy to Borrow: {}", self.easy_to_borrow)?;
        writeln!(f, "  Fractionable: {}", self.fractionable)?;
        Ok(())
    }
}

impl Asset {
    pub fn from_json(json: &str) -> JsonResult<Asset> {
        serde_json::from_str(json)
    }

    pub fn to_json(json: &str) -> JsonResult<Vec<Asset>> {
        serde_json::from_str(json)
    }
}
