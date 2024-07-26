use tokio::runtime::Runtime;

pub fn get_latest_price(symbol: impl Into<String>) -> Result<f64, Box<dyn std::error::Error>> {
    let provider = yahoo_finance_api::YahooConnector::new()?;
    let response = Runtime::new()?
        .block_on(provider.get_latest_quotes(symbol.into().as_str(), "1d"))?
        .last_quote()?;

    Ok(response.close)
}

pub fn is_symbol_valid(symbol: impl Into<String>) -> bool {
    let provider = yahoo_finance_api::YahooConnector::new().unwrap();
    let response = Runtime::new()
        .unwrap()
        .block_on(provider.get_latest_quotes(symbol.into().as_str(), "1d"));

    response.is_ok()
}
