mod alpaca;
mod auth;
mod endpoint;
mod params;
mod error;
mod client;
mod account;
mod query;

use serde::Deserialize;
use alpaca::Alpaca;
use crate::account::TradingAccount;
use crate::query::Query;
#[derive(Debug, Deserialize)]
struct User {
    account_number: String,
    currency: String,
}

fn main() {
    let client = Alpaca::new(
        "paper-api.alpaca.markets",
        "",
        "",
    )
    .unwrap();
    let endpoint = TradingAccount::builder().build().unwrap();
    let user: User  = endpoint.query(&client).unwrap();
    println!("{:#?}", user);
}
