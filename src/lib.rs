use crate::types::{
    CheckOrderResponse, ClientResponse, DegiroClient, FavoritesResponse, HistoryResponse, Order,
    OrderConfirmationResponse, PortfolioResponse, ProductInfo, ProductInfoResponse,
    ProductSearchResponse, TotpLoginRequest, TotpLoginResponse, TransactionsHistoryResponse,
};
use jiff::civil::Date;
use log::LevelFilter;
use reqwest::{Client, RequestBuilder};
use totp_rs::{Algorithm, Secret, TOTP};

pub mod error;
pub mod types;

use error::DegiroError;
use types::{AccountInfo, AccountInfoResponse, AccountOverview, AccountOverviewResponse};
type Result<T> = std::result::Result<T, DegiroError>;

impl DegiroClient {
    pub fn finalize(self) -> Result<Self> {
        let connection_verbose = self.log_level <= LevelFilter::Debug;

        let client = Client::builder()
            .cookie_store(true)
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:138.0) Gecko/20100101 Firefox/138.0",
            )
            .connection_verbose(connection_verbose)
            .build()?;

        Ok(Self { client, ..self })
    }

    fn session_and_account(&self) -> Result<(&str, u64)> {
        let session_id = self
            .session_id
            .as_deref()
            .ok_or(DegiroError::MissingSessionId)?;
        let int_account = self.int_account.ok_or(DegiroError::MissingIntAccount)?;

        Ok((session_id, int_account))
    }

    fn build_get(&self, url: &str) -> RequestBuilder {
        self.client
            .get(url)
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://trader.degiro.nl/trader/")
    }

    fn build_post(&self, url: &str) -> RequestBuilder {
        self.client
            .post(url)
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Referer", "https://trader.degiro.nl/trader/")
    }

    pub async fn login_with_totp(&mut self) -> Result<()> {
        let url = "https://trader.degiro.nl/login/secure/login/totp";

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Encoded(self.totp_secret.clone())
                .to_bytes()
                .map_err(|_| DegiroError::InvalidTotpSecret)?,
        )?;
        let totp_token = totp.generate_current()?;

        let totp_payload = TotpLoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
            query_params: serde_json::json!({}),
            one_time_password: totp_token,
            save_device: false,
        };

        let req = self
            .build_post(url)
            .header("Origin", "https://trader.degiro.nl")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Referer", "https://trader.degiro.nl/login/nl")
            .json(&totp_payload)
            .build()?;

        let res = self.client.execute(req).await?;
        let totp_response: TotpLoginResponse = res.json().await?;

        self.session_id = Some(totp_response.session_id.clone());
        self.get_int_account().await
    }

    pub async fn get_int_account(&mut self) -> Result<()> {
        let url = format!(
            "https://trader.degiro.nl/pa/secure/client?sessionId={}",
            self.session_id
                .as_deref()
                .ok_or(DegiroError::MissingSessionId)?
        );

        let response = self.build_get(&url).send().await?;
        let client_response: ClientResponse = response.json().await?;
        self.int_account = Some(client_response.data.int_account);

        Ok(())
    }

    pub async fn get_favorites(&self) -> Result<Vec<u64>> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/favorites/secure/v1?intAccount={}&sessionId={}",
            int_account, session_id
        );

        let response = self.build_get(&url).send().await?;
        let fav_response: FavoritesResponse = response.json().await?;
        Ok(fav_response.data.first().unwrap().product_ids.clone())
    }

    pub async fn get_products_details(&self, ids: Vec<String>) -> Result<Vec<ProductInfo>> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/product_search/secure/v5/products/info?intAccount={}&sessionId={}",
            int_account, session_id
        );

        let response = self.build_post(&url).json(&ids).send().await?;
        let product_info: ProductInfoResponse = response.json().await?;
        Ok(product_info.data.into_values().collect())
    }

    pub async fn search_product_by_name(&self, name: &str) -> Result<Vec<ProductInfo>> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/product_search/secure/v5/products/lookup?offset=0&limit=10&searchText={}&intAccount={}&sessionId={}",
            name, int_account, session_id
        );

        let response = self.build_get(&url).send().await?;
        let found_products: ProductSearchResponse = response.json().await?;
        Ok(found_products.products)
    }

    pub async fn get_portfolio(&self) -> Result<PortfolioResponse> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/update/{};jsessionid={}?intAccount={}&jsessionId={}&portfolio=0",
            int_account, session_id, int_account, session_id
        );

        let response = self.build_get(&url).send().await?;
        let res: PortfolioResponse = response.json().await?;
        Ok(res)
    }

    pub async fn get_order_history(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> Result<HistoryResponse> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = "https://trader.degiro.nl/portfolio-reports/secure/v4/order-history";
        let params = [
            ("fromDate", from_date),
            ("toDate", to_date),
            ("intAccount", &int_account.to_string()),
            ("sessionId", session_id),
        ];

        let response = self.build_get(url).query(&params).send().await?;
        let res: HistoryResponse = response.json().await?;
        Ok(res)
    }

    pub async fn check_order(&self, order: &Order) -> Result<CheckOrderResponse> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/checkOrder;jsessionid={}",
            session_id
        );

        let response = self
            .build_post(&url)
            .query(&[
                ("intAccount", int_account.to_string()),
                ("sessionId", session_id.to_string()),
            ])
            .json(order)
            .send()
            .await?;

        let res: CheckOrderResponse = response.json().await?;
        Ok(res)
    }

    // TODO: test this :)
    pub async fn confirm_order(
        &self,
        confirmation_id: String,
        order: &Order,
    ) -> Result<OrderConfirmationResponse> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/order/{};jsessionid={}",
            confirmation_id, session_id
        );

        let response = self
            .build_post(&url)
            .query(&[
                ("intAccount", int_account.to_string()),
                ("sessionId", session_id.to_string()),
            ])
            .json(order)
            .send()
            .await?;

        let res: OrderConfirmationResponse = response.json().await?;
        Ok(res)
    }

    pub async fn get_transaction_history(
        &self,
        from_date: &Date,
        to_date: &Date,
        aggregate_order: bool,
    ) -> Result<TransactionsHistoryResponse> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = "https://trader.degiro.nl/portfolio-reports/secure/v4/transactions";

        let params = [
            ("fromDate", from_date.to_string()),
            ("toDate", to_date.to_string()),
            ("groupTransactionsByOrder", aggregate_order.to_string()),
            ("intAccount", int_account.to_string()),
            ("sessionId", session_id.to_string()),
        ];

        let response = self.build_get(url).query(&params).send().await?;
        let json: TransactionsHistoryResponse = response.json().await?;
        Ok(json)
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = format!(
            "https://trader.degiro.nl/trading/secure/v5/account/info/{};jsessionid={}",
            int_account, session_id
        );

        let response = self.build_get(&url).send().await?;
        let json: AccountInfoResponse = response.json().await?;

        Ok(json.data)
    }

    pub async fn get_account_overview(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> Result<AccountOverview> {
        let (session_id, int_account) = self.session_and_account()?;
        let url = "https://trader.degiro.nl/portfolio-reports/secure/v6/accountoverview";
        let params = [
            ("fromDate", from_date),
            ("toDate", to_date),
            ("intAccount", &int_account.to_string()),
            ("sessionId", &session_id.to_string()),
        ];

        let response = self.build_get(&url).query(&params).send().await?;
        let json: AccountOverviewResponse = response.json().await?;

        Ok(json.data)
    }
}
