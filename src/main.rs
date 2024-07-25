mod command;
mod utils;
mod yahoo_api;

use std::env::current_dir;

use clap::Command;
use utils::watchlist::Watchlist;
use yahoo_api::get_latest_price;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(unused_variables)]
    let current_dir = current_dir()?;
    #[allow(unused_variables)]
    let watchlist_path = current_dir.join("watchlist.json");
    #[allow(unused_variables)]
    let portfolio_path = current_dir.join("portfolio.json");

    let matches = Command::new("octopustock")
        .subcommand(command::stock_command())
        .subcommand(command::watchlist_command())
        .subcommand(command::portfolio_command())
        .get_matches();

    if let Some(stock_cmd) = matches.subcommand_matches("stock") {
        let symbol = stock_cmd.get_one::<String>("symbol").unwrap();
        println!("Price: {:.2} $", get_latest_price(symbol)?);
    } else if let Some(watchlist_cmd) = matches.subcommand_matches("watchlist") {
        let mut watchlist = Watchlist::new(r"D:\project\octopustock\watchlist.json".into());

        if let Some(add_cmd) = watchlist_cmd.subcommand_matches("add") {
            let symbol = add_cmd.get_one::<String>("symbol").unwrap();

            println!("Adding {} to watchlist", symbol);
            watchlist.add(symbol.clone());
        } else if let Some(remove_cmd) = watchlist_cmd.subcommand_matches("remove") {
            let symbol = remove_cmd.get_one::<String>("symbol").unwrap();

            println!("Removing {} from watchlist", symbol);
            watchlist.remove(symbol);
        } else if watchlist_cmd.subcommand_matches("list").is_some() {
            for (idx, stock) in watchlist.stocks.iter().enumerate() {
                println!("[{}] {} : {} $", idx, stock, get_latest_price(stock)?);
            }
        }

        watchlist.save("watchlist.json".into())?;
    } else if let Some(portfolio_cmd) = matches.subcommand_matches("portfolio") {
        if let Some(set_cmd) = portfolio_cmd.subcommand_matches("set") {
            let symbol = set_cmd.get_one::<String>("symbol").unwrap();
            let quantity = set_cmd.get_one::<usize>("quantity").unwrap();
            println!("Setting {} to portfolio with quantity {}", symbol, quantity);
        } else if let Some(remove_cmd) = portfolio_cmd.subcommand_matches("remove") {
            let symbol = remove_cmd.get_one::<String>("symbol").unwrap();
            println!("Removing {} from portfolio", symbol);
        }
    } else {
        println!("No subcommand found");
    }

    Ok(())
}
