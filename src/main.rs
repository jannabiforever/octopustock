mod command;
use clap::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("octopustock")
        .subcommand(command::stock_command())
        .get_matches();

    match matches.subcommand_matches("stock") {
        Some(symbol) => {
            let symbol = symbol
                .get_one::<String>("symbol")
                .expect("No symbol provided");

            let provider = yahoo_finance_api::YahooConnector::new()?;
            let response = tokio::runtime::Runtime::new()?
                .block_on(provider.get_latest_quotes("AAPL", "1d"))?
                .last_quote()?;

            println!("Symbol: {}", symbol);
            println!("Price: {:.2} $", response.close);
        }
        None => {
            println!("No subcommand found");
        }
    }

    Ok(())
}
