use std::io::Write;
use std::{collections::HashMap, path::PathBuf};

use crate::yahoo_api::get_latest_price;

pub struct Portfolio {
    pub stocks: HashMap<String, usize>,
}

impl Portfolio {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        if let Ok(portfolio) = Self::try_load(path) {
            portfolio
        } else {
            Self {
                stocks: HashMap::new(),
            }
        }
    }

    fn try_load(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path.into())?;
        let stocks: HashMap<String, usize> = serde_json::from_reader(file)?;

        Ok(Self { stocks })
    }

    pub fn save(&self, path: impl Into<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = std::fs::File::create(path.into())?;

        let json = serde_json::to_string(&self.stocks)?;
        wtr.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn set(&mut self, symbol: impl Into<String>, quantity: impl Into<usize>) {
        self.stocks.insert(symbol.into(), quantity.into());
    }

    pub fn remove(&mut self, symbol: impl Into<String>) {
        self.stocks.remove(&symbol.into());
    }

    pub fn value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let total = self.stocks.iter().fold(0.0, |acc, (stock, &quantity)| {
            let price = get_latest_price(stock).unwrap_or(0.0);
            acc + (price * quantity as f64)
        });

        Ok(total)
    }
}
