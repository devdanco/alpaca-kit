mod alpaca;
mod auth;
mod endpoint;
mod params;
mod error;
mod client;
mod options_contract;
mod account;
mod asset;
mod query;
mod raw;

use serde::Deserialize;
use alpaca::Alpaca;
use crate::account::TradingAccount;
use crate::asset::Asset;
use crate::options_contract::OptionsContract;
use crate::query::Query;
use crate::raw::raw;


#[derive(Debug, Deserialize)]
struct User {
    account_number: String,
    currency: String,
}

#[derive(Debug, Deserialize)]
struct Options {
    symbol: String
}

#[derive(Debug, Deserialize)]
struct OptionsStruct {
    option_contracts: Vec<Options>
}

#[derive(Debug, Deserialize)]
struct AssetId {
    class: String
}
fn main() {
    let client = Alpaca::new(
        "paper-api.alpaca.markets",
        "",
        "",
    )
    .unwrap();
    let endpoint = TradingAccount::builder().build().unwrap();
    // let user: User  = endpoint.query(&client).unwrap();
    
    let raw_data_user = raw(endpoint).query(&client).unwrap();
    
    let options = OptionsContract::builder().build().unwrap();
    let contract: OptionsStruct = options.query(&client).unwrap();

    let asset = Asset::builder().symbol_or_asset_id("AAPL".to_string()).build();
    let aaa: AssetId = asset.query(&client).unwrap();
}
