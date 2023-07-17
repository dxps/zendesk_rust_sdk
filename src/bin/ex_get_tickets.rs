use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.get_tickets().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(resp) = result {
        println!("Got {} tickets.", resp.tickets.len());
        if resp.tickets.len() > 0 {
            println!("Here it is:");
            for item in &resp.tickets {
                println!("{:?}\n", item)
            }
        }
        println!(
            "Get Tickets Summary\n-------------------\n\t- items: {}",
            resp.tickets.len()
        );
    } else {
        println!("Failed to list the tickets: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
