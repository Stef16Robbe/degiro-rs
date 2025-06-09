use crate::types::{
    ClientResponse, DegiroClient, FavoritesResponse, ProductInfo, ProductInfoResponse,
    TotpLoginRequest, TotpLoginResponse,
};
use anyhow::Result;
use reqwest::{Client, cookie::Jar};
use std::sync::Arc;
use totp_rs::{Algorithm, Secret, TOTP};
use types::{CheckOrderResponse, HistoryResponse, Order, PortfolioResponse, ProductSearchResponse};

pub mod types;

impl DegiroClient {
    pub fn new(username: String, password: String, totp_secret: String) -> Result<Self> {
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
            int_account: None,
        })
    }

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
        let client_url = format!(
            "https://trader.degiro.nl/pa/secure/client?sessionId={}",
            self.session_id.clone().unwrap()
        );

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
        let url = format!(
            "https://trader.degiro.nl/favorites/secure/v1?intAccount={}&sessionId={}",
            self.int_account.unwrap(),
            self.session_id.clone().unwrap()
        );

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
    pub async fn get_products_details(&self, ids: Vec<String>) -> Result<Vec<ProductInfo>> {
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

    pub async fn search_product_by_name(&self, name: &str) -> Result<Vec<ProductInfo>> {
        let url = format!(
            "https://trader.degiro.nl/product_search/secure/v5/products/lookup?offset=0&limit=10&searchText={}&intAccount={}&sessionId={}",
            name,
            self.int_account.unwrap(),
            self.session_id.clone().unwrap()
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await?;

        let found_products: ProductSearchResponse = response.json().await?;
        Ok(found_products.products)
    }

    pub async fn get_portfolio(&self) -> Result<PortfolioResponse> {
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/update/{};jsessionid={}?intAccount={}&jsessionId={}&portfolio=0",
            self.int_account.unwrap(),
            self.session_id.clone().unwrap(),
            self.int_account.unwrap(),
            self.session_id.clone().unwrap(),
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await?;

        let res: PortfolioResponse = response.json().await?;
        Ok(res)
    }

    /// Retrieves the user's historical order data from the DEGIRO `/order-history` endpoint.
    ///
    /// This returns both open and completed orders placed within the specified date range,
    /// including buy/sell type, size, price, order status, and more.
    ///
    /// # Arguments
    ///
    /// * `from_date` - The start date in `dd/mm/yyyy` format (e.g., `"01/01/2024"`).
    /// * `to_date` - The end date in `dd/mm/yyyy` format (e.g., `"01/01/2025"`).
    ///
    /// # Returns
    ///
    /// A [`HistoryResponse`] containing a list of historical orders (which may be empty).
    ///
    /// # Notes
    ///
    /// - This only returns manual orders placed via the DEGIRO interface (not transactions).
    ///
    /// # Example
    ///
    /// ```rust
    /// let history = client.get_order_history("01/01/2024", "01/01/2025").await?;
    /// for item in history.data {
    ///     println!("Order: {:?} {} @ {}", item.buysell, item.size, item.price);
    /// }
    /// ```
    pub async fn get_order_history(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> Result<HistoryResponse> {
        let url = "https://trader.degiro.nl/portfolio-reports/secure/v4/order-history";

        let params = [
            ("fromDate", from_date),
            ("toDate", to_date),
            ("intAccount", &self.int_account.unwrap().to_string()),
            ("sessionId", &self.session_id.clone().unwrap()),
        ];

        let response = self
            .client
            .get(url)
            .query(&params)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await?;

        let res: HistoryResponse = response.json().await?;

        Ok(res)
    }

    pub async fn check_order(&self, order: &Order) -> Result<CheckOrderResponse> {
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/checkOrder;jsessionid={}",
            self.session_id.clone().unwrap()
        );

        let params = [
            ("intAccount", self.int_account.unwrap().to_string()),
            ("sessionId", self.session_id.clone().unwrap()),
        ];

        let response = self
            .client
            .post(&url)
            .query(&params)
            .json(order)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .send()
            .await?;

        let res: CheckOrderResponse = response.json().await?;

        Ok(res)
    }
}
