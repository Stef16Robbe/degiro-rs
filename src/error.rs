use std::time::SystemTimeError;

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DegiroError {
    #[error("network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("TOTP generation failed: {0}")]
    Totp(#[from] totp_rs::TotpUrlError),

    #[error("invalid TOTP secret encoding")]
    InvalidTotpSecret,

    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("missing required session ID")]
    MissingSessionId,

    #[error("missing required int account")]
    MissingIntAccount,

    #[error("HTTP error: {status} - {body}")]
    HttpError { status: StatusCode, body: String },

    #[error("TOTP time error: {0}")]
    Time(#[from] SystemTimeError),
}
