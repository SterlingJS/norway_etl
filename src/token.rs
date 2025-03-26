use serde::{Deserialize, Serialize};
use system::time::{UNIX_EPOCH, SystemTime};

const CLIENT_ID: &str = "CLIENT_ID";
const CLIENT_SECRET: &str = "CLIENT_SECRET";
const AIS_BASE_URL: &str = "AIS_BASE_URL";
const TOKEN_ENDPOINT: &str = "TOKEN_ENDPOINT";

#[derive(Serialize, Deserialize, Debug)]
struct TokenRequstData {
    client_id: String,
    client_secret: String,
    grant_type: String,
    scope: String
}

struct TokenResponse {
    access_token: String,
    expires_in: Int32,
    token_type: String,
    scope: String
}

pub struct TokenCredential {
    token: Option<String>,
    expires: Option<SystemTime>
}

impl TokenCredential {
    pub async fn get_token(&mut self) -> String {
        // If we have a valid token that hasn't expired, return that!
        let now = SystemTime::now();
        if (self.token.is_some() && self.expires.is_some() && self.expires.cmp(SystemTime::now()) == 1) {
            return self.token;
        }

        // We must need to fetch a new token
        let client_id: String = std::env::var(CLIENT_ID).expect("CLIENT_ID must be set");
        let client_secret: String = std::env::var(CLIENT_SECRET).expect("CLIENT_SECRET must be set");
        let ais_base_url: String = std::env::var(AIS_BASE_URL).expect("AIS_BASE_URL must be set");
        let token_endpoint: String = std::env::var(TOKEN_ENDPOINT).expect("TOKEN_ENDPOINT must be set");
        let _streaming_data_endpoint: String = ais_base_url.clone() + "/live/v1/latest/ais";

        let start = SystemTime::now();

        let client = reqwest::Client::new();
    
        let data = TokenRequstData {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            grant_type: "client_credentials".into(),
            scope: "ais".into()
        };
    
        // Here we are passing a reference to the serde_urlencoded so data is just being borrowed
        // The values can't be updated and we will get back the same data object that went in
        let data = serde_urlencoded::to_string(&data).expect("serialize issue");
    
        let response: String = client
            .post(token_endpoint)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(data)
            .send()
            .await
            .expect("failed to get token response")
            .text()
            .await
            .expect("failed to get payload");
        
        let token_response: TokenResponse = serde_json::from_str(&response).unwrap();
        self.expires = SystemTime::now();
        self.expires.checked_add(Duration::new(token.expires_in - 60, 0));
        self.token = token_response.token;

        self.token
    }
}