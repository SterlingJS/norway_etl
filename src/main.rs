use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenRequstData {
    client_id: String,
    client_secret: String,
    grant_type: String,
    scope: String
}

async fn get_token() -> String {
    const CLIENT_ID: &str = "CLIENT_ID";
    const CLIENT_SECRET: &str = "CLIENT_SECRET";
    const AIS_BASE_URL: &str = "AIS_BASE_URL";
    const TOKEN_ENDPOINT: &str = "TOKEN_ENDPOINT";
    let client_id: String = std::env::var(CLIENT_ID).expect("CLIENT_ID must be set");
    let client_secret: String = std::env::var(CLIENT_SECRET).expect("CLIENT_SECRET must be set");
    let ais_base_url: String = std::env::var(AIS_BASE_URL).expect("AIS_BASE_URL must be set");
    let token_endpoint: String = std::env::var(TOKEN_ENDPOINT).expect("TOKEN_ENDPOINT must be set");
    let _streaming_data_endpoint: String = ais_base_url.clone() + "/live/v1/latest/ais";

    // println!("client_id: {client_id}");
    // println!("client_secret: {client_secret}");
    // println!("base_url: {ais_base_url}");
    // println!("token_endpoint: {token_endpoint}");
    // println!("streaming_data_endpoint: {_streaming_data_endpoint}");

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

    response
}

// This function will start up in a container as a chron job, run every 15min
// The function should execute immediately and avoid sleeping  when possible
// Once the database has been updated, this function can exit
#[async_std::main]
async fn main() {
    println!("Hello, world!");

    // Load the environment using a .env
    dotenv().ok();
    
    // Await the token fetch and print
    let token_response: String = get_token().await;
    println!("{token_response}");


    // Extract the data from our Norway API
        // Static fetch: Get the latest results since the last time we ran a scan (15min in the past)
        // Filter messages that aren't type 1, 2, or 3
        // Also handle type 18 position messages
        
        // {
            //   "geometry": {
            //     "type": "Polygon",
            //     "coordinates": [
            //       [
            //         [
            //           10.4,
            //           60.5
            //         ]
            //       ],
            //       [
            //         [
            //           10.4,
            //           60.5
            //         ]
            //       ],
            //       [
            //         [
            //           10.4,
            //           60.5
            //         ]
            //       ],
            //       [
            //         [
            //           10.4,
            //           60.5
            //         ]
            //       ]
            //     ]
            //   },
                // "since": "2025-03-25T23:50:25.338Z",
            //   "mmsi": [
            //     0
            //   ],
            //   "shipTypes": [
            //     0
            //   ],
            //   "countryCodes": [
            //     "FIN"
            //   ],
            //   "includePosition": true
            //   "includeStatic": false,
            //   "includeAton": false,
            //   "includeSafetyRelated": false,
            //   "includeBinaryBroadcastMetHyd": false
            // }

    // Transform the data into the needed format
    // Are there any missing fields we need to run follow up queries for?
    // Anything we should drop / types to change / things to calculate?

    // Load the data back to our database
    // Insert the latest entries into our position database table
}
