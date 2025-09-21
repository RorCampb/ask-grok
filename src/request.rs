pub mod toolcall;


use toolcall::{ToolCall, ToolResponse, Function};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use reqwest_eventsource::{EventSource, Event};

#[derive(Debug, Serialize, Clone)]
pub struct Body {
    model: String,
    messages: Vec<Messages>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Function>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    pub role: String,
    pub content: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolResponse>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choices>,
    pub system_fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub index: u32,
    pub delta: Option<Delta>,
    pub message: Option<Messages>
}


impl Body {
    pub fn new(model:impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            stream: true,
            tools: None,
        }
    }

    pub fn add_message(&mut self, role: impl Into<String>, content: impl Into<String>) {
        self.messages.push(Messages {
            role: role.into(),
            content: content.into(),
            tool_calls: None,
        });
    }

    pub fn tools(&mut self, tools: impl IntoIterator<Item = ToolCall>) -> &mut Self {
        let functions = tools.into_iter().map(|tool_call| Function {
            r#type: "function".into(),
            function: tool_call,
        });
        self.tools = Some(functions.collect());
        self
    }
    
    pub fn send(&self, client: &Client, api_key: &str) -> EventSource {
        let grok_response = client
            .post("https://api.x.ai/v1/chat/completions")
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .json(&self);

        EventSource::new(grok_response).expect("Failed to start events...")
    }

    pub async fn run_tool(&self, client: &Client, api_key: &str) -> Result<Vec<ToolResponse>, reqwest::Error> {
        let mut body = self.clone();
        body.stream = false;

        let grok_response = client
            .post("https://api.x.ai/v1/chat/completions")
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let raw = grok_response.text().await?;
        println!("Raw TOOL Response: {}", raw);
        let parsed: Data = serde_json::from_str(&raw).expect("Failed to parse tool calls");
        
        let tool_calls: Vec<ToolResponse> = parsed
            .choices
            .get(0)
            .and_then(|c| c.message.as_ref())
            .and_then(|m| m.tool_calls.clone())
            .unwrap_or_default();
        if tool_calls.is_empty() {
            println!("No tool calls found");
        }
        else {
            println!("ToolCall Result: {:?}", tool_calls);
        }
        
        Ok(tool_calls)

    }
}
