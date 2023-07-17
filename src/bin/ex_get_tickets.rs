use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let result = client.get_tickets().await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(dto) = result {
        println!("Got {} tickets.", dto.tickets.len());
        if dto.tickets.len() > 0 {
            println!("Here it is:");
            for item in &dto.tickets {
                println!("{:?}\n", item)
            }
        }
        println!(
            "Get Tickets Summary\n-------------------\n\t- items: {}",
            dto.tickets.len()
        );
    } else {
        println!("Failed to list the tickets: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
