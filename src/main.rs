use dotenv::dotenv;
use zendesk_rsdk::{AuthCredential::ApiTokenCredential, Client};

#[tokio::main]
async fn main() {
    // Load the environment variables (including from .env file).
    dotenv().ok();

    let base_url = std::env::var("ZENDESK_URL").expect("ZENDESK_URL env var is not defined");
    let email = std::env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL env var is not defined");
    let api_token =
        std::env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN env var is not defined");

    let zc = Client::new(base_url, ApiTokenCredential { email, api_token });

    let groups = zc.list_groups().await;
    println!("{:?}", groups);
}
