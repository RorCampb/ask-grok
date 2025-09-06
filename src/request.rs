use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct Body {
    model: String,
    messages: Vec<Messages>, 
    stream: bool,
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
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub index: u32,
    pub delta: Delta,
}


impl Body {
    pub fn new(model:impl Into<String>) -> Body {
        Body {
            model: model.into(),
            messages: Vec::new(),
            stream: true
        }
    }

    pub fn add_message(&mut self, role: impl Into<String>, content: impl Into<String>) {
        self.messages.push(Messages {
            role: role.into(),
            content: content.into()
        });
    }
}
