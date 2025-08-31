use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use reqwest::Client;
use reqwest_eventsource::{EventSource, Event};
use std::env;
use futures_util::StreamExt;


#[derive(Debug, Serialize, Deserialize)]
struct ToolCall  {
    name: String,
    description: String,
    parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parameters {
    r#type: String,
    properties: HashMap<String, Properties>,
    required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Properties {
    r#type: String,
    description: String,
}


#[derive(Debug, Serialize)]
struct Body {
    model: String,
    messages: Vec<Messages>, 
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Messages {
    role: String,
    content: String, 
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: String,
    object: String,
    created: u32,
    model: String,
    choices: Vec<Choices>,
    system_fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Delta {
    content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choices {
    index: u32,
    delta: Delta,
}

fn ask_grok(client: &Client, api_key: &str, request: &Body) -> EventSource {
    let grok_response = client
        .post("https://api.x.ai/v1/chat/completions")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(request);

    EventSource::new(grok_response).expect("Failed to start events...")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = env::var("XAI_API_KEY")?;
    let client = Client::new();
    
    let mut request_body: Body = Body {
        messages: vec![
            Messages {
                role: "system".into(),
                content: "You are a helpful CLI asssistant.".into(),
            },
        ],
        stream: true,
        model: "grok-4".to_string() 
    }; 
    
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
        request_body.messages.push(
            Messages {
                role: "user".into(),
                content: grok_input.trim().to_string(),
            }
        );

        let mut es = ask_grok(&client, &api_key, &request_body);
        
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
        request_body.messages.push(
            Messages {
                role: "assistant".into(),
                content: grok_response.clone(),
            }
        );

        es.close();
    }
    Ok(())
}
