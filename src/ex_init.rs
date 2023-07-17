use crate::{AuthCredential::ApiTokenCredential, Client};
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[clap(name = "Get Tickets", about = "Example of how to get the tickets.")]
struct CliFlags {
    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,
}

pub fn ex_init() -> Client {
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

    let base_url = std::env::var("ZENDESK_URL").expect("ZENDESK_URL env var is not defined");
    let email = std::env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL env var is not defined");
    let api_token = std::env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN env var is not defined");

    // Initing the client.
    Client::new(base_url, ApiTokenCredential { email, api_token })
}
