#![allow(dead_code)]
#![allow(unused_imports)]
use bon::{Builder, builder};
use jiff::civil::DateTime;
use log::LevelFilter;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};

#[derive(Builder)]
pub struct DegiroClient {
    #[builder(skip)]
    pub client: Client,
    pub(crate) username: String,
    pub(crate) password: String,
    /// For now just assuming everyone is reasonable
    /// and uses 2FA for their investments(!!!)
    pub(crate) totp_secret: String,
    pub(crate) session_id: Option<String>,
    pub(crate) int_account: Option<u64>,
    #[builder(default = LevelFilter::Off)]
    pub(crate) log_level: LevelFilter,
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

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct HistoryItem {
//     pub buysell: BuySell,

//     pub created: DateTime,

//     pub current_traded_size: i32,

//     pub active: bool,

//     pub last: DateTime,

//     #[serde(default)]
//     pub order_id: Option<String>,

//     pub order_time_type_id: OrderTimeType,

//     pub order_type_id: OrderType,

//     pub price: f64,

//     pub product_id: i32,

//     pub size: i32,

//     pub status: String,

//     pub stop_price: f64,

//     pub total_traded_size: i32,

//     #[serde(rename = "type")]
//     pub event_type: String,
// }

#[derive(Debug, Deserialize)]
/// From: https://github.com/Chavithra/degiro-connector/blob/bffe906194a6f3e91fafdfb8830efa894e8751a8/degiro_connector/trading/models/order.py#L151
/// not sure why it's just 'B' or 'S' for History but for an order it's [OrderAction]
pub enum BuySell {
    B,
    S,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
/// From: https://github.com/Chavithra/degiro-connector/blob/bffe906194a6f3e91fafdfb8830efa894e8751a8/degiro_connector/trading/models/order.py#L30-L34
/// not sure what these unknown types are...
pub enum OrderTimeType {
    GoodTillCanceled = 3,
    GoodTillDay = 1,
    Unknown0 = 0,
    Unknown2 = 2,
    Unknown = -1, // fallback
}

impl Serialize for OrderTimeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for OrderTimeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i32::deserialize(deserializer)?;
        Ok(match v {
            0 => OrderTimeType::Unknown0,
            1 => OrderTimeType::GoodTillDay,
            2 => OrderTimeType::Unknown2,
            3 => OrderTimeType::GoodTillCanceled,
            _ => OrderTimeType::Unknown,
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
/// From: https://github.com/Chavithra/degiro-connector/blob/bffe906194a6f3e91fafdfb8830efa894e8751a8/degiro_connector/trading/models/order.py#L14-L27
/// TODO: Find out remaining types & whether they are relevant
pub enum OrderType {
    Limit = 0,
    StopLimit = 1,
    Market = 2,
    StopLoss = 3,
    Unknown = -1, // fallback
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i32::deserialize(deserializer)?;
        Ok(match v {
            0 => OrderType::Limit,
            1 => OrderType::StopLimit,
            2 => OrderType::Market,
            3 => OrderType::StopLoss,
            _ => OrderType::Unknown,
        })
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderAction {
    Buy,
    Sell,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub buy_sell: OrderAction,
    pub order_type: OrderType,
    pub product_id: String,
    pub size: f64,
    pub price: f64,
    pub time_type: OrderTimeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CheckOrderResponse {
    pub data: OrderCheck,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCheck {
    pub auto_fx_conversion_rate: Option<f64>,
    pub confirmation_id: String,
    pub free_space_new: Option<f64>,
    pub response_datetime: Option<DateTime>,
    pub request_duration: Option<Duration>,
    pub transaction_auto_fx_opposite_surcharges: Option<Vec<Value>>,
    pub transaction_auto_fx_surcharges: Option<Vec<Value>>,
    pub transaction_fee: Option<f64>,
    pub transaction_fees: Option<Vec<Value>>,
    pub transaction_opposite_fees: Option<Vec<Value>>,
    pub transaction_taxes: Option<Vec<Value>>,
    pub show_ex_ante_report_link: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct OrderConfirmationResponse {
    pub data: OrderConfirmation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmation {
    order_id: String,
    response_datetime: Option<DateTime>,
    request_duration: Option<Duration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub auto_fx_fee_in_base_currency: Option<f64>,
    pub buysell: Option<BuySell>,
    pub counter_party: Option<String>,
    pub date: Option<DateTime>,
    pub executing_entity_id: Option<String>,
    pub fee_in_base_currency: Option<f64>,
    pub fx_rate: Option<f64>,
    pub gross_fx_rate: Option<f64>,
    pub id: Option<i64>,
    pub nett_fx_rate: Option<f64>,
    pub order_type_id: Option<i64>,
    pub price: Option<f64>,
    pub product_id: Option<i64>,
    pub quantity: Option<i64>,
    pub total: Option<f64>,
    pub total_fees_in_base_currency: Option<f64>,
    pub total_in_base_currency: Option<f64>,
    pub total_plus_all_fees_in_base_currency: Option<f64>,
    pub total_plus_fee_in_base_currency: Option<f64>,
    pub transfered: Option<bool>,
    pub trading_venue: Option<String>,
    pub transaction_type_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsHistoryResponse {
    pub data: Vec<HistoryItem>,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponse {
    pub data: AccountInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub base_currency: String,
    pub cash_funds: HashMap<String, Vec<CashFund>>,
    pub client_id: u64,
    pub currency_pairs: HashMap<String, CurrencyPair>,
    // TODO: this can be an enum if we know the possible values
    pub margin_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CashFund {
    pub flatex: bool,
    pub id: u64,
    pub name: String,

    #[serde(rename = "productIds")]
    pub product_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyPair {
    pub id: i64, // Some are -1 or -2, so signed
    pub price: String,
}
