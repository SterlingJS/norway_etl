use dotenv::dotenv;


async fn print_token() -> Result<String, Box<dyn Error>> {
    const CLIENT_ID: &str = "CLIENT_ID";
    const CLIENT_SECRET: &str = "CLIENT_SECRET";
    const AIS_BASE_URL: &str = "AIS_BASE_URL";
    let client_id: String = std::env::var(CLIENT_ID).expect("CLIENT_ID must be set");
    let client_secret: String = std::env::var(CLIENT_SECRET).expect("CLIENT_SECRET must be set");
    let ais_base_url: String = std::env::var(AIS_BASE_URL).expect("AIS_BASE_URL must be set");
    let token_endpoint: String = ais_base_url.clone() + "/connect/token";
    let _streaming_data_endpoint: String = ais_base_url.clone() + "/live/v1/latest/ais";


    let client = reqwest::Client::new();

    let future = client
        .post(token_endpoint)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(format!("client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials&scope=ais"))
        .send();
    
    future.await?;
    
    Ok(())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");


    dotenv().ok();
    
    print_token().await;
    // let resp = client
    //     .get(streaming_data_endpoint)
    //     .header("Authorization": )
    // println!("{:#?}", resp);
    Ok(())
}
