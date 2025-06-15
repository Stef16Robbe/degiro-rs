use degiro_rs::types::DegiroClient;
use httpmock::{
    Method::{GET, POST},
    MockServer,
};
use serde_json::json;

const DEGIRO_USERNAME: &str = "TEST";
const DEGIRO_PASSWORD: &str = "TEST";
const DEGIRO_TOTP_SECRET: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";

struct TestSetup {
    server: MockServer,
    client: DegiroClient,
}

impl TestSetup {
    async fn new() -> Self {
        let server = MockServer::start_async().await;

        let client = DegiroClient::builder()
            .username(DEGIRO_USERNAME.to_string())
            .password(DEGIRO_PASSWORD.to_string())
            .totp_secret(DEGIRO_TOTP_SECRET.to_string())
            .base_url(server.base_url())
            .finalize();

        Self { server, client }
    }
}

#[tokio::test]
async fn login_success() {
    let mut setup = TestSetup::new().await;

    let totp_mock = setup
        .server
        .mock_async(|when, then| {
            when.method(POST).path("/login/secure/login/totp");
            then.status(200)
                .header("content-type", "application/json;charset=UTF-8")
                .json_body(json!({
                    "captchaRequired": false,
                    "isPassCodeEnabled": true,
                    "locale": "en_US",
                    "redirectUrl": "https://trader.degiro.nl/trader/",
                    "sessionId": "mock-session-123",
                    "status": 0,
                    "statusText": "success",
                    "userTokens": []
                }));
        })
        .await;

    let client_mock = setup
        .server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/pa/secure/client")
                .query_param("sessionId", "mock-session-123");
            then.status(200)
                .header("content-type", "application/json;charset=UTF-8")
                .json_body(json!({
                    "data": {
                        "intAccount": 12345678,
                        "username": "testuser",
                        "email": "testuser@example.com"
                    }
                }));
        })
        .await;

    setup.client.login_with_totp().await.unwrap();

    totp_mock.assert();
    client_mock.assert();
}
