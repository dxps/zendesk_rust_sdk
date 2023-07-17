use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.list_organizations().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(resp) = result {
        println!("Got {} organizations:", resp.organizations.len());
        for item in &resp.organizations {
            println!(" {:?}", item)
        }
        println!(
            "\nGet Organizations Summary\n-------------------------\n\t- items: {}",
            resp.organizations.len()
        );
    } else {
        eprintln!("Failed to get the organizations: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
