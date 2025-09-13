mod toolcall;


use toolcall::ToolCall;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use reqwest_eventsource::{EventSource, Event};

#[derive(Debug, Serialize)]
pub struct Body {
    model: String,
    messages: Vec<Messages>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    role: String,
    content: String, 
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
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub index: u32,
    pub delta: Delta,
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
            content: content.into()
        });
    }

    pub fn tools(mut self, tools: impl IntoIterator<Item = ToolCall>) -> Self {
        self.tools = Some(tools.into_iter().collect());
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

    pub async fn run_tool(&self, client: &Client, api_key: &str) -> Result<Data, reqwest::Error> {
        let grok_response = client
            .post("https://api.x.ai/v1/chat/completions")
            .bearer_auth(api_key)
            .header("Content-Type", "application/json")
            .json(&self)
            .send()
            .await?;
        let parsed: Data = grok_response.json().await?
        let parsed: Data = serde_json::from_str(&grok_response_body).expect("Failed to read tool response body");

        if let Some(tool_call) = parsed.choices.tool_calls
        
        Ok(parsed)
    }
}
