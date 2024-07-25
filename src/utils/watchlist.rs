use std::io::Write;
use std::path::PathBuf;

pub struct Watchlist {
    pub stocks: Vec<String>,
}

impl Watchlist {
    pub fn new(path: PathBuf) -> Self {
        if let Ok(watchlist) = Self::try_load(path) {
            watchlist
        } else {
            Self { stocks: vec![] }
        }
    }

    fn try_load(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rdr = std::fs::File::open(path)?;
        let stocks: Vec<String> = serde_json::from_reader(&mut rdr)?;

        Ok(Self { stocks })
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr =
            std::fs::File::open(&path).unwrap_or_else(|_| std::fs::File::create(&path).unwrap());

        let json = serde_json::to_string(&self.stocks)?;
        wtr.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn add(&mut self, symbol: String) {
        self.stocks.push(symbol);
    }

    pub fn remove(&mut self, symbol: &str) {
        self.stocks.retain(|s| s != symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_watchlist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("watchlist.json");

        let mut watchlist = Watchlist::new(path.clone());
        assert_eq!(watchlist.stocks.len(), 0);

        watchlist.add("AAPL".to_string());
        assert_eq!(watchlist.stocks.len(), 1);

        watchlist.add("GOOGL".to_string());
        assert_eq!(watchlist.stocks.len(), 2);

        watchlist.remove("AAPL");
        assert_eq!(watchlist.stocks.len(), 1);

        watchlist.save(path.clone()).unwrap();
    }
}
