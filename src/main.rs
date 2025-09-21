mod request;

use request::{Body, Data};
use request::toolcall::{ToolCall, Parameters, Args};
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
                    if let Some(choice) = delta
                        .choices
                        .get(0) {
                        if let Some(text) = choice
                            .delta.as_ref()
                            .and_then(|d| d.content.as_deref()) {
                                print!("{text}");
                                grok_response.push_str(&text);
                        }
                    }
                    else { continue;}
                }
                Err(e) => {
                    println!("Error in streaming response: {e}");
                    break;
                }
            }
        }
        let mut toolspec = Parameters::new();
        toolspec.add_property(
            "toolcall_test",
            Args::string("write something simple for testing purposes. Need to see if tool call worked."),
            true,
            );
        let toolcall = ToolCall::new(
            "toolcall_test",
            "testing to see if my system implemnts a toolcall. Please comply",
            toolspec
            );
        request_body.tools(vec![toolcall]);
        match request_body.run_tool(&client, &api_key).await {
            Ok(tools) => {
                if tools.is_empty() {
                    println!("No tools returned...");
                } else {
                        for tool in tools {
                            println!("TOOL RESPONSE: {:#?}", tool);
                        }
                    }
            }
            Err(e) => {
                println!("Error calling tools: {:?}", e);
            }
        }
        request_body.add_message("assistant", grok_response.clone());
        

        es.close();
    }
    Ok(())
}
