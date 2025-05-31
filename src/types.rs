#![allow(dead_code)]
#![allow(unused_imports)]

use reqwest::{Client, cookie::Jar};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

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
    pub(crate) isin: String,
    pub(crate) symbol: String,
    pub(crate) contract_size: f64,
    pub(crate) product_type: String,
    pub(crate) product_type_id: u32,
    pub(crate) tradable: bool,
    pub(crate) category: String,
    pub(crate) currency: String,
    pub(crate) active: bool,

    // Optional fields that may not be present in all products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) strike_price: Option<f64>,

    pub(crate) exchange_id: String,
    pub(crate) only_eod_prices: bool,

    // Trading-related optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order_time_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) buy_order_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sell_order_types: Option<Vec<String>>,

    // Price information
    pub(crate) close_price: f64,
    pub(crate) close_price_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_shortable: Option<bool>,

    // Feed and data quality
    pub(crate) feed_quality: String,
    pub(crate) order_book_depth: u32,
    pub(crate) vwd_identifier_type: String,
    pub(crate) vwd_id: String,
    pub(crate) quality_switchable: bool,
    pub(crate) quality_switch_free: bool,
    pub(crate) vwd_module_id: u32,

    // Secondary feed data (optional)
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

    // Special product attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) product_bit_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProductSearchResponse {
    pub(crate) offset: u64,
    pub(crate) products: Vec<ProductInfo>,
}

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
