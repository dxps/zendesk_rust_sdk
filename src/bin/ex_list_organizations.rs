use clap::Parser;
use dotenv::dotenv;
use zendesk_rsdk::{AuthCredential::ApiTokenCredential, Client};

#[tokio::main]
async fn main() {
    let cli_flags = CliFlags::parse();

    // Load the environment variables (including from .env file).
    dotenv().ok();

    // Logging init.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{},hyper=info,mio=info,reqwest=warn,sqlx=warn,tower_http=warn",
                cli_flags.log_level
            ),
        )
    }
    tracing_subscriber::fmt::init();

    println!("RUST_LOG={}", std::env::var("RUST_LOG").unwrap());

    let base_url = std::env::var("ZENDESK_URL").expect("ZENDESK_URL env var is not defined");
    let email = std::env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL env var is not defined");
    let api_token =
        std::env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN env var is not defined");

    // Initing the client.
    let zc = Client::new(base_url, ApiTokenCredential { email, api_token });

    // Using the client.
    let result = zc.list_organizations().await;
    log::debug!("\n___________________________");
    if let Ok(dto) = result {
        println!("Got {} organizations:", dto.organizations.len());
        for item in dto.organizations {
            println!(" {:?}", item)
        }
    } else {
        eprintln!(
            "Failed to list the organizations: {}",
            result.err().unwrap()
        )
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "List Organizations",
    about = "Example of listing the organizations."
)]
struct CliFlags {
    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,
}
