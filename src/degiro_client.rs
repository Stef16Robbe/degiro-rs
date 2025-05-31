#![allow(dead_code)]
use std::collections::HashMap;
use reqwest::{Client, cookie::Jar};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use totp_rs::{Algorithm, Secret, TOTP};
use anyhow::Result;

#[derive(Debug, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
    #[serde(rename = "queryParams")]
    query_params: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct TotpLoginRequest {
    username: String,
    password: String,
    #[serde(rename = "queryParams")]
    query_params: serde_json::Value,
    #[serde(rename = "oneTimePassword")]
    one_time_password: String,
    #[serde(rename = "saveDevice")]
    save_device: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "captchaRequired")]
    pub captcha_required: bool,
    pub status: i32,
    #[serde(rename = "statusText")]
    pub status_text: String,
}

#[derive(Debug, Deserialize)]
pub struct TotpLoginResponse {
    #[serde(rename = "captchaRequired")]
    captcha_required: bool,
    #[serde(rename = "isPassCodeEnabled")]
    is_pass_code_enabled: bool,
    locale: String,
    #[serde(rename = "redirectUrl")]
    redirect_url: String,
    #[serde(rename = "sessionId")]
    session_id: String,
    pub status: i32,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "userTokens")]
    user_tokens: Vec<serde_json::Value>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientResponse {
    pub data: ClientData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientData {
    #[serde(rename = "intAccount")]
    int_account: u64,
    username: String,
    email: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FavoritesResponse {
    pub data: Vec<FavoritesData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoritesData {
    #[serde(rename = "productIds")]
    pub product_ids: Vec<u64>
}

#[derive(Debug, Serialize)]
struct ProductRequest {
    // #[serde(rename = "productIds")]
    products: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductInfoResponse {
    pub data: HashMap<String, ProductInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    pub id: String,
    pub name: String,
    pub isin: String,
    pub symbol: String,
    pub contract_size: f64,
    pub product_type: String,
    pub product_type_id: u32,
    pub tradable: bool,
    pub category: String,
    pub currency: String,
    pub active: bool,
    
    // Optional fields that may not be present in all products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    
    pub exchange_id: String,
    pub only_eod_prices: bool,
    
    // Trading-related optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_time_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_order_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_order_types: Option<Vec<String>>,
    
    // Price information
    pub close_price: f64,
    pub close_price_date: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shortable: Option<bool>,
    
    // Feed and data quality
    pub feed_quality: String,
    pub order_book_depth: u32,
    pub vwd_identifier_type: String,
    pub vwd_id: String,
    pub quality_switchable: bool,
    pub quality_switch_free: bool,
    pub vwd_module_id: u32,
    
    // Secondary feed data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_quality_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_book_depth_secondary: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vwd_identifier_type_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vwd_id_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_switchable_secondary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_switch_free_secondary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vwd_module_id_secondary: Option<u32>,
    
    // Special product attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_bit_types: Option<Vec<String>>,
}

// If you want to work with individual products more easily:
impl ProductInfoResponse {
    /// Get all products as a Vec instead of HashMap
    pub fn into_products(self) -> Vec<ProductInfo> {
        self.data.into_values().collect()
    }
    
    /// Get a specific product by ID
    pub fn get_product(&self, id: &str) -> Option<&ProductInfo> {
        self.data.get(id)
    }
    
    /// Get only tradable products
    pub fn tradable_products(&self) -> Vec<&ProductInfo> {
        self.data.values().filter(|p| p.tradable).collect()
    }
    
    /// Get products by type
    pub fn products_by_type(&self, product_type: &str) -> Vec<&ProductInfo> {
        self.data.values()
            .filter(|p| p.product_type == product_type)
            .collect()
    }
}

pub struct DegiroClient {
    client: Client,
    username: String,
    password: String,
    /// For now just assuming everyone is reasonable
    /// and uses 2FA for their investments(!!!)
    totp_secret: String,
    jar: Option<Arc<Jar>>,
    session_id: Option<String>,
    int_account: Option<u64>,
}

impl DegiroClient {
    pub fn new(
        username: String,
        password: String,
        totp_secret: String,
    ) -> Result<Self> {
        // Create a cookie jar to store cookies (like JSESSIONID)
        let jar = Arc::new(Jar::default());

        // Build the HTTP client with the cookie jar
        let client = Client::builder()
            .cookie_provider(jar.clone())
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:138.0) Gecko/20100101 Firefox/138.0",
            )
            .build()?;

        Ok(DegiroClient {
            client,
            username,
            password,
            totp_secret,
            jar: Some(jar),
            session_id: None,
            int_account: None
        })
    }

    // pub async fn login(&mut self) -> Result<LoginResponse> {
    //     let login_url = "https://trader.degiro.nl/login/secure/login";

    //     let login_payload = LoginRequest {
    //         username: self.username.clone(),
    //         password: self.password.clone(),
    //         query_params: serde_json::json!({}),
    //     };

    //     let response = self
    //         .client
    //         .post(login_url)
    //         .header("Content-Type", "application/json;charset=UTF-8")
    //         .header("Accept", "application/json, text/plain, */*")
    //         .header("Origin", "https://trader.degiro.nl")
    //         .header("Referer", "https://trader.degiro.nl/login/nl")
    //         .json(&login_payload)
    //         .send()
    //         .await?;

    //     let login_response: LoginResponse = response.json().await?;

    //     Ok(login_response)
    // }

    pub async fn login_with_totp(&mut self) -> Result<()> {
        let totp_url = "https://trader.degiro.nl/login/secure/login/totp";

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(self.totp_secret.clone())
                .to_bytes()
                .unwrap(),
        )
        .unwrap();
        let totp_token = totp.generate_current().unwrap();
        // dbg!(&totp_token);

        let totp_payload = TotpLoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
            query_params: serde_json::json!({}),
            one_time_password: totp_token,
            save_device: false,
        };

        let response = self
            .client
            .post(totp_url)
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Accept", "application/json, text/plain, */*")
            .header("Origin", "https://trader.degiro.nl")
            .header("Referer", "https://trader.degiro.nl/login/nl")
            .json(&totp_payload)
            .send()
            .await?;

        let totp_response: TotpLoginResponse = response.json().await?;

        // Store session ID for future requests
        self.session_id = Some(totp_response.session_id.clone());

        // Now we must get the `intaccount`
        self.get_int_account().await?;

        Ok(())
    }

    pub async fn get_int_account(&mut self) -> Result<()> {
        let client_url = format!("https://trader.degiro.nl/pa/secure/client?sessionId={}", self.session_id.clone().unwrap());

        let response = self
            .client
            .get(client_url)
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Accept", "application/json, text/plain, */*")
            // .header("Origin", "https://trader.degiro.nl")
            .header("Referer", "https://trader.degiro.nl/trader/")
            .send()
            .await?;

        let client_response: ClientResponse = response.json().await?;
        self.int_account = Some(client_response.data.int_account);

        Ok(())
    }

    pub async fn get_favorites(&self) -> Result<Vec<u64>> {
        let url = format!("https://trader.degiro.nl/favorites/secure/v1?intAccount={}&sessionId={}", self.int_account.unwrap(), self.session_id.clone().unwrap());

        let response = self
            .client
            .get(url)
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Accept", "application/json, text/plain, */*")
            // .header("Origin", "https://trader.degiro.nl")
            .header("Referer", "https://trader.degiro.nl/trader/")
            .send()
            .await?;

        let fav_response: FavoritesResponse = response.json().await?;
        Ok(fav_response.data.first().unwrap().product_ids.clone())
    }

    // TODO: use https://docs.rs/serde_path_to_error/0.1.17/serde_path_to_error/
    pub async fn get_product_details(&self, ids: Vec<u64>) -> Result<Vec<ProductInfo>> {
        let url = format!(
            "https://trader.degiro.nl/product_search/secure/v5/products/info?intAccount={}&sessionId={}",
            self.int_account.unwrap(),
            self.session_id.clone().unwrap()
        );

        let response = self
            .client
            .post(&url)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&ids)
            .send()
            .await?;

        let product_info: ProductInfoResponse = response.json().await?;
        Ok(product_info.data.into_values().collect())
    }
}
