mod command;
mod utils;
mod yahoo_api;

use std::env::current_dir;

use clap::Command;
use utils::{portfolio::Portfolio, watchlist::Watchlist};
use yahoo_api::{get_latest_price, is_symbol_valid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("octopustock")
        .subcommand(command::stock_command())
        .subcommand(command::watchlist_command())
        .subcommand(command::portfolio_command())
        .get_matches();

    match matches.subcommand() {
        Some(("stock", sub_m)) => {
            let stock = sub_m
                .get_one::<String>("stock")
                .expect("Stock symbol is required")
                .to_uppercase();

            let price = get_latest_price(&stock)?;
            println!("{}: ${:.2}", stock, price);
        }
        Some(("watchlist", sub_m)) => {
            let file_dir = current_dir()?.join("watchlist.json");
            let mut watchlist = Watchlist::new(&file_dir);

            match sub_m.subcommand() {
                Some(("add", add_m)) => {
                    let symbol = add_m
                        .get_one::<String>("symbol")
                        .expect("Stock symbol is required")
                        .to_uppercase();

                    if is_symbol_valid(&symbol) {
                        println!("Adding {} to watchlist", symbol);
                        watchlist.add(&symbol);
                    } else {
                        println!("Invalid stock symbol");
                    }                    
                }
                Some(("remove", remove_m)) => {
                    let stock = remove_m
                        .get_one::<String>("symbol")
                        .expect("Stock symbol is required")
                        .to_uppercase();

                    println!("Removing {} from watchlist", stock);
                    watchlist.remove(&stock);
                }
                Some(("list", _)) => {
                    if watchlist.stocks.is_empty() {
                        println!("Watchlist is empty");
                        return Ok(());
                    }

                    println!("Stocks in watchlist:");
                    for (idx, stock) in watchlist.stocks.iter().enumerate() {
                        println!(
                            "[{}] {} : {:.2} $",
                            idx + 1,
                            stock,
                            get_latest_price(stock)?
                        );
                    }
                }
                _ => {
                    println!("No subcommand found");
                }
            }

            watchlist.save(&file_dir)?;
        }
        Some(("portfolio", sub_m)) => {
            let file_path = current_dir()?.join("portfolio.json");
            let mut portfolio = Portfolio::new(&file_path);

            match sub_m.subcommand() {
                Some(("set", set_m)) => {
                    let symbol = set_m
                        .get_one::<String>("symbol")
                        .expect("Stock symbol is required")
                        .to_uppercase();

                    let quantity = set_m
                        .get_one::<usize>("quantity")
                        .expect("Quantity is required");

                    if is_symbol_valid(&symbol) {
                        println!("Adding {} to portfolio with quantity {}", symbol, quantity);
                        portfolio.set(&symbol, *quantity);
                    } else {
                        println!("Invalid stock symbol");
                    }
                }
                Some(("remove", remove_m)) => {
                    let stock = remove_m
                        .get_one::<String>("symbol")
                        .expect("Stock symbol is required")
                        .to_uppercase();

                    println!("Removing {} from portfolio", stock);
                    portfolio.remove(&stock);
                }
                Some(("list", _)) => {
                    println!("Stocks in portfolio:\n");

                    if portfolio.stocks.is_empty() {
                        println!("Portfolio is empty");
                        return Ok(());
                    }

                    for (idx, (stock, &quantity)) in portfolio.stocks.iter().enumerate() {
                        let price = get_latest_price(stock)?;
                        println!(
                            "[{}] {} : {} shares \nvalue per share : {:.2} $ \n(value : {:.2} $)\n",
                            idx+1,
                            stock,
                            quantity,
                            price,
                            price * quantity as f64
                        );
                    }
                }
                Some(("value", _)) => {
                    println!("Total value of portfolio: {:.2} $", portfolio.value()?);
                }
                _ => {
                    println!("No subcommand found");
                }
            }

            portfolio.save(&file_path)?;
        }
        _ => {
            println!("No subcommand found");
        }
    }

    Ok(())
}
