use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

pub struct Watchlist {
    pub stocks: HashSet<String>,
}

impl Watchlist {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        if let Ok(watchlist) = Self::try_load(path) {
            watchlist
        } else {
            Self {
                stocks: HashSet::new(),
            }
        }
    }

    fn try_load(path: impl Into<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path.into())?;
        let stocks: HashSet<String> = serde_json::from_reader(file)?;

        Ok(Self { stocks })
    }

    pub fn save(&self, path: impl Into<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = std::fs::File::create(path.into())?;

        let json = serde_json::to_string(&self.stocks)?;
        wtr.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn add(&mut self, symbol: impl Into<String>) {
        self.stocks.insert(symbol.into());
    }

    pub fn remove(&mut self, symbol: impl Into<String>) {
        self.stocks.remove(&symbol.into());
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

        watchlist.add("AAPL");
        assert_eq!(watchlist.stocks.len(), 1);

        watchlist.add("GOOGL");
        assert_eq!(watchlist.stocks.len(), 2);

        watchlist.remove("AAPL");
        assert_eq!(watchlist.stocks.len(), 1);

        watchlist.save(path.clone()).unwrap();
    }
}
