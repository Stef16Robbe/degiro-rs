#![allow(dead_code)]
#![allow(unused_imports)]
use anyhow::Result;
use degiro_rs::types::{DegiroClient, Order};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let username = env::var("DEGIRO_USERNAME").expect("username environment variable not set.");
    // Note to self. dotenv interprets certain chars like shell variables! escape them
    let password = env::var("DEGIRO_PASSWORD").expect("password environment variable not set.");
    let totp_secret =
        env::var("DEGIRO_TOTP_SECRET").expect("totp secret environment variable not set.");

    let mut client = DegiroClient::new(username, password, totp_secret)?;

    client.login_with_totp().await?;

    // let res = client.get_favorites().await?;
    // let favs = client
    //     .get_products_details(vec![String::from("USD")])
    //     .await?;
    // for f in favs {
    //     println!("{}", f.name);
    // }

    // let found = client.search_product_by_name("rhm").await?;
    // for f in found {
    //     println!("{}", f.name);
    // }

    // let port = client.get_portfolio().await?;
    // let ids: Vec<String> = port.portfolio.value.iter().map(|v| v.id.clone()).collect();
    // let det = client.get_products_details(ids).await?;
    // for d in det {
    //     println!("{}", d.name);
    // }

    // let res = client.get_order_history("01/01/2024", "09/06/2025").await?;
    // dbg!(res);

    // let order = Order {
    //     buy_sell: degiro_rs::types::OrderAction::Buy,
    //     order_type: degiro_rs::types::OrderType::Market,
    //     product_id: "1819819".to_string(),
    //     size: 1.0,
    //     price: 1.0,
    //     time_type: degiro_rs::types::OrderTimeType::GoodTillCanceled,
    //     stop_price: None,
    // };
    // let checked = client.check_order(&order).await?;
    // dbg!(&checked);
    // let confirmed = client.confirm_order(&checked.confirmation_id, &order).await?;
    // dbg!(&confirmed);

    Ok(())
}
