mod request;

use request::{Body, Data};
use reqwest::Client;
use reqwest_eventsource::Event;
use serde_json;
use std::env;
use futures_util::StreamExt;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = env::var("XAI_API_KEY")?;
    let client = Client::new();
   
    let mut request_body = Body::new("grok-4");
    request_body.add_message("system", "You are a helpful CLI assistant");
     
    let mut grok_input = String::new();
    let mut grok_response = String::new();

    loop {
        grok_input.clear();
        grok_response.clear();
        
        std::io::stdin().read_line(&mut grok_input).unwrap();
        
        if grok_input.trim().to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }
        request_body.add_message("user", grok_input.trim().to_string());
        

        let mut es = request_body.send(&client, &api_key);
        
        while let Some(event) = es.next().await {
            match event {
                Ok(Event::Open) => {}

                Ok(Event::Message(msg)) => {
                    if msg.data == "[DONE]" {
                        break;
                    }

                    let delta: Data = serde_json::from_str(&msg.data).expect("Failed to read delta");
                    if let Some(chunk) = delta
                        .choices
                        .get(0)
                        .and_then(|c| c.delta.content.as_deref()) 
                    {
                        print!("{chunk}");
                        grok_response.push_str(&chunk);
                    } else { continue; }
                }
                Err(e) => {
                    println!("Error in streaming response: {e}");
                    break;
                }
            }
        }
        request_body.add_message("assistant", grok_response.clone());
        

        es.close();
    }
    Ok(())
}
