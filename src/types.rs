#![allow(dead_code)]
#![allow(unused_imports)]

use jiff::civil::DateTime;
use reqwest::{cookie::Jar, Client};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, sync::Arc};

pub struct DegiroClient {
    pub client: Client,
    pub(crate) username: String,
    pub(crate) password: String,
    /// For now just assuming everyone is reasonable
    /// and uses 2FA for their investments(!!!)
    pub(crate) totp_secret: String,
    pub(crate) jar: Option<Arc<Jar>>,
    pub(crate) session_id: Option<String>,
    pub(crate) int_account: Option<u64>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TotpLoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
    #[serde(rename = "queryParams")]
    pub(crate) query_params: serde_json::Value,
    #[serde(rename = "oneTimePassword")]
    pub(crate) one_time_password: String,
    #[serde(rename = "saveDevice")]
    pub(crate) save_device: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TotpLoginResponse {
    #[serde(rename = "captchaRequired")]
    captcha_required: bool,
    #[serde(rename = "isPassCodeEnabled")]
    is_pass_code_enabled: bool,
    locale: String,
    #[serde(rename = "redirectUrl")]
    redirect_url: String,
    #[serde(rename = "sessionId")]
    pub(crate) session_id: String,
    pub(crate) status: i32,
    #[serde(rename = "statusText")]
    pub(crate) status_text: String,
    #[serde(rename = "userTokens")]
    user_tokens: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ClientResponse {
    pub(crate) data: ClientData,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ClientData {
    #[serde(rename = "intAccount")]
    pub(crate) int_account: u64,
    pub(crate) username: String,
    pub(crate) email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FavoritesResponse {
    pub(crate) data: Vec<FavoritesData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FavoritesData {
    #[serde(rename = "productIds")]
    pub(crate) product_ids: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProductInfoResponse {
    pub(crate) data: HashMap<String, ProductInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub currency: String,
    pub contract_size: f64,
    pub close_price: f64,
    pub product_type_id: u32,
    pub tradable: bool,

    // Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) isin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) strike_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exchange_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) only_eod_prices: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order_time_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) buy_order_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sell_order_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) close_price_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_shortable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) feed_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order_book_depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_identifier_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quality_switchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quality_switch_free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_module_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) feed_quality_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order_book_depth_secondary: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_identifier_type_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_id_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quality_switchable_secondary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quality_switch_free_secondary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vwd_module_id_secondary: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) product_bit_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProductSearchResponse {
    pub(crate) offset: u64,
    pub(crate) products: Vec<ProductInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub portfolio: Portfolio,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    #[serde(rename = "lastUpdated")]
    pub last_updated: u64,

    pub name: String,

    pub value: Vec<PositionRow>,

    #[serde(rename = "isAdded")]
    pub is_added: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionRow {
    pub name: String,

    /// A String because currencies have their value as ID
    /// e.g. "USD"
    pub id: String,

    pub value: Vec<PositionField>,

    #[serde(rename = "isAdded")]
    pub is_added: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionField {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueField>,

    #[serde(rename = "isAdded")]
    pub is_added: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueField {
    String(String),
    Number(f64),
    Object(HashMap<String, f64>),
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    pub data: Vec<HistoryItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub buysell: BuySell,

    pub created: DateTime,

    pub current_traded_size: i32,

    pub active: bool,

    pub last: DateTime,

    #[serde(default)]
    pub order_id: Option<String>,

    pub order_time_type_id: OrderTimeType,

    pub order_type_id: OrderType,

    pub price: f64,

    pub product_id: i32,

    pub size: i32,

    pub status: String,

    pub stop_price: f64,

    pub total_traded_size: i32,

    #[serde(rename = "type")]
    pub event_type: String,
}

#[derive(Debug, Deserialize)]
pub enum BuySell {
    B,
    S,
}

#[derive(Debug, Deserialize)]
#[repr(i32)]
#[serde(rename_all = "UPPERCASE")]
/// From: https://github.com/Chavithra/degiro-connector/blob/bffe906194a6f3e91fafdfb8830efa894e8751a8/degiro_connector/trading/models/order.py#L30-L34
/// not sure what these unknown types are...
pub enum OrderTimeType {
    GoodTillCanceled = 3,
    GoodTillDay = 1,
    Unknown0 = 0,
    Unknown2 = 2,
}

#[derive(Debug, Deserialize)]
#[repr(i32)]
/// From: https://github.com/Chavithra/degiro-connector/blob/bffe906194a6f3e91fafdfb8830efa894e8751a8/degiro_connector/trading/models/order.py#L14-L27
/// TODO: Find out remaining types & whether they are relevant
pub enum OrderType {
    Limit = 0,
    Market = 2,
    StopLimit = 1,
    StopLoss = 3,
    #[serde(other)]
    Unknown,
}
