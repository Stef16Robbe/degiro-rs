use degiro_client::DegiroClient;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

mod degiro_client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let username = env::var("DEGIRO_USERNAME").expect("username environment variable not set.");
    // Note to self. dotenv interprets certain chars like shell variables! escape them
    let password  = env::var("DEGIRO_PASSWORD").expect("password environment variable not set.");
    let totp_secret = env::var("DEGIRO_TOTP_SECRET").expect("totp secret environment variable not set.");

    let mut client = DegiroClient::new(username, password, totp_secret)?;

    // If 2FA is needed (status 6 means 2FA needed)
    // if login_response.status == 6 && login_response.status_text == "totpNeeded"

    client.login_with_totp().await?;
    // println!("2FA login response: {:?}", totp_login_response);

    // If login successful, make an example authenticated request
    // if totp_login_response.status == 0 && totp_login_response.status_text == "success" {
    println!("Getting favorites...");
    let res = client.get_favorites().await?;
    // dbg!(&res);
    // }

    let favs = client.get_product_details(res.clone()).await?;
    dbg!(favs);

    Ok(())
}
