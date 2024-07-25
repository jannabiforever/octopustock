use clap::{value_parser, Arg, Command};

pub fn stock_command() -> Command {
    Command::new("stock")
        .about("Get stock's latest price")
        .arg(Arg::new("symbol").required(true).long("symbol").short('s'))
}

pub fn watchlist_command() -> Command {
    Command::new("watchlist")
        .about("Manage watchlist")
        .subcommand(
            Command::new("add")
                .about("Add a stock to the watchlist")
                .arg(Arg::new("symbol").required(true).long("symbol").short('s')),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a stock from the watchlist")
                .arg(Arg::new("symbol").required(true).long("symbol").short('s')),
        )
        .subcommand(Command::new("list").about("List all stocks in the watchlist"))
}

pub fn portfolio_command() -> Command {
    Command::new("portfolio")
        .about("Manage portfolio")
        .subcommand(
            Command::new("add")
                .about("Add a stock to the portfolio")
                .arg(Arg::new("symbol").required(true).long("symbol").short('s'))
                .arg(
                    Arg::new("quantity")
                        .required(true)
                        .long("quantity")
                        .short('q')
                        .value_parser(value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a stock from the portfolio")
                .arg(Arg::new("symbol").required(true).long("symbol").short('s')),
        )
        .subcommand(Command::new("list").about("List all stocks in the portfolio"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_command() {
        let stock = stock_command().get_matches_from(vec!["stock", "--symbol", "AAPL"]);
        let symbol = stock.get_one::<String>("symbol").unwrap();
        assert_eq!(symbol, "AAPL");
    }

    #[test]
    fn test_watchlist_command() {
        let add =
            watchlist_command().get_matches_from(vec!["watchlist", "add", "--symbol", "AAPL"]);
        let symbol = add
            .subcommand_matches("add")
            .unwrap()
            .get_one::<String>("symbol")
            .unwrap();
        assert_eq!(symbol, "AAPL");

        let remove =
            watchlist_command().get_matches_from(vec!["watchlist", "remove", "--symbol", "AAPL"]);
        let symbol = remove
            .subcommand_matches("remove")
            .unwrap()
            .get_one::<String>("symbol")
            .unwrap();

        assert_eq!(symbol, "AAPL");

        let list = watchlist_command().get_matches_from(vec!["watchlist", "list"]);
        assert!(list.subcommand_matches("list").is_some());
    }

    #[test]
    fn test_portfolio_command() {
        let add = portfolio_command().get_matches_from(vec![
            "portfolio",
            "add",
            "--symbol",
            "AAPL",
            "--quantity",
            "10",
        ]);
        let symbol = add
            .subcommand_matches("add")
            .unwrap()
            .get_one::<String>("symbol")
            .unwrap();
        let quantity = add
            .subcommand_matches("add")
            .unwrap()
            .get_one::<usize>("quantity")
            .unwrap();
        assert_eq!(symbol, "AAPL");
        assert_eq!(*quantity, 10);

        let remove = portfolio_command().get_matches_from(vec!["portfolio", "remove", "--symbol", "AAPL"]);
        let symbol = remove
            .subcommand_matches("remove")
            .unwrap()
            .get_one::<String>("symbol")
            .unwrap();
        assert_eq!(symbol, "AAPL");

        let list = portfolio_command().get_matches_from(vec!["portfolio", "list"]);
        assert!(list.subcommand_matches("list").is_some());
    }
}
