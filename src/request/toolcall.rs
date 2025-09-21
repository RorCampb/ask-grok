use serde::{Serialize, Deserialize};
use std::collections::HashMap;




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolResponse {
    pub id: String,
    pub function: ToolResult,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolResult {
    pub name: String,
    pub arguments: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub r#type: String,
    pub function: ToolCall,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameters {
    pub r#type: String,
    pub properties: HashMap<String, Args>,
    pub required: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Args {
    pub r#type: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}


impl ToolCall {
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: Parameters) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: parameters,
        }
    }
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            r#type: "object".into(),
            properties: HashMap::new(),
            required: Vec::new(),
        }
    }

    pub fn add_property(&mut self, name: impl Into<String>, args: Args, required: bool) -> &mut Self {
        let key = name.into();
        if required {
            self.required.push(key.clone());
        }
        self.properties.insert(key, args);
        self
    }
}

impl Args {
    pub fn string(description: impl Into<String>) -> Self {
        Self {
            r#type: "string".into(),
            description: description.into(),
            r#enum: None,
            default: None,
        }
    }

    pub fn with_enum(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.r#enum = Some(values.into_iter().map(Into::into).collect());
        self
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }
}
