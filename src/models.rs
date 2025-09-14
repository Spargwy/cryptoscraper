use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PricesResponse = HashMap<String, Currency>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Currency {
    pub usd: f64,
}
