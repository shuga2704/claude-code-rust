use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ToolName {
    Read,
    Write,
    Bash,
}

#[derive(Serialize, Deserialize, Debug)]
struct Property {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    #[serde(rename = "type")]
    type_: String,
    properties: HashMap<String, Property>,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Function {
    name: String,
    description: String,
    parameters: Parameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentTool {
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolMessage {
    role: String,
    tool_call_id: String,
    content: String,
}

impl AgentTool {
    pub fn read() -> Self {
        serde_json::from_str(include_str!("../tools/read.json")).unwrap()
    }

    pub fn write() -> Self {
        serde_json::from_str(include_str!("../tools/write.json")).unwrap()
    }

    pub fn bash() -> Self {
        serde_json::from_str(include_str!("../tools/bash.json")).unwrap()
    }
}

impl ToolMessage {
    pub fn new(tool_call_id: String, content: Option<String>) -> Self {
        ToolMessage {
            role: "tool".to_string(),
            tool_call_id,
            content: content.unwrap_or_default(),
        }
    }
}
