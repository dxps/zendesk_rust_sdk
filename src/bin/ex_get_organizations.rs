use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.list_organizations().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(dto) = result {
        println!("Got {} organizations:", dto.organizations.len());
        for item in &dto.organizations {
            println!(" {:?}", item)
        }
        println!(
            "\nGet Organizations Summary\n-------------------------\n\t- items: {}",
            dto.organizations.len()
        );
    } else {
        eprintln!("Failed to get the organizations: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
