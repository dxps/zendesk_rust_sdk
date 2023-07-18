use zendesk_rsdk::ex_init;

#[tokio::main]
async fn main() {
    let client = ex_init();

    let query = format!("type:ticket status:open created>2022-01-01");
    let result = client.search_tickets(&query).await;

    println!("--------------------------------------------------------------------------------\n");
    if let Ok(result) = result {
        println!("Using query '{query}', found {} tickets.", result.len());
        if result.len() > 0 {
            println!("\nHere is the result:");
            for item in &result {
                println!("{:?}\n", item)
            }
        }
        println!("Search Tickets Summary\n----------------------");
        println!("\t- items: {}", result.len());
        println!("\t- query: '{query}'");
    } else {
        println!("Failed to search tickets: {}", result.err().unwrap())
    }
    println!("\n--------------------------------------------------------------------------------");
}
