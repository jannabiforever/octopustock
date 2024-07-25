pub fn get_latest_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let provider = yahoo_finance_api::YahooConnector::new()?;
    let response = tokio::runtime::Runtime::new()?
        .block_on(provider.get_latest_quotes(symbol, "1d"))?
        .last_quote()?;

    Ok(response.close)
}
