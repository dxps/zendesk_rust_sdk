use clap::Parser;
use dotenv::dotenv;
use zendesk_rsdk::{AuthCredential::ApiTokenCredential, Client};

#[tokio::main]
async fn main() {
    // Logging init.
    let opt = CliOptions::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{},hyper=info,mio=info,sqlx=warn,tower_http=warn",
                opt.log_level
            ),
        )
    }

    // Load the environment variables (including from .env file).
    dotenv().ok();
    let base_url = std::env::var("ZENDESK_URL").expect("ZENDESK_URL env var is not defined");
    let email = std::env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL env var is not defined");
    let api_token =
        std::env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN env var is not defined");

    let zc = Client::new(base_url, ApiTokenCredential { email, api_token });

    let result = zc.list_groups().await;
    println!("\n___________________________");
    if let Ok(group_res) = result {
        println!("Got {} groups:", group_res.groups.len());
        for group in group_res.groups {
            println!(" {:?}", group)
        }
    } else {
        eprintln!("Failed to list the groups: {}", result.err().unwrap())
    }
}

#[derive(Parser, Debug)]
#[clap(name = "sample1", about = "todo-doc")]
struct CliOptions {
    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,
}
