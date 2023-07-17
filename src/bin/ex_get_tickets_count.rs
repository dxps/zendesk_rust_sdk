use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.get_tickets_count().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(resp) = result {
        println!(
            "Get Tickets Count Summary:\n\t- items: {}\n\t- refreshed at: {}",
            resp.count.value, resp.count.refreshed_at
        );
    } else {
        println!("Failed to get the tickets count: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
