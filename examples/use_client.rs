use anyhow::Result;
use degiro_rs::types::DegiroClient;
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

    let res = client.get_order_history("01/01/2024", "09/06/2025").await?;
    dbg!(res);

    Ok(())
}
