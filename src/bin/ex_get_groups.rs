use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.list_groups().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(resp) = result {
        println!("Got {} groups.\n", resp.groups.len());
        for item in &resp.groups {
            println!("{:?}\n", item)
        }
        println!(
            "Get Groups Summary\n------------------\n\t- items: {}",
            resp.groups.len()
        );
    } else {
        eprintln!("Failed to get the groups: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
