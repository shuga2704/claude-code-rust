use std::{collections::HashMap, fs::read_to_string};

use serde::{Deserialize, Serialize};

use crate::{
    message::Message,
    tool::{ToolMessage, ToolName},
};

pub struct ResponseError {
    pub message: String,
}

impl From<std::io::Error> for ResponseError {
    fn from(error: std::io::Error) -> ResponseError {
        ResponseError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for ResponseError {
    fn from(error: serde_json::Error) -> ResponseError {
        ResponseError {
            message: error.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    name: ToolName,
    arguments: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    index: u32,
    message: ResponseMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Responses {
    choices: Vec<Response>,
}

impl Responses {
    pub fn execute(&self) -> Result<Vec<Message>, ResponseError> {
        self.choices.iter().next().unwrap().message.execute()
    }

    pub fn from_value(value: serde_json::Value) -> Result<Self, ResponseError> {
        Ok(serde_json::from_value(value)?)
    }

    pub fn is_finished(&self) -> bool {
        self.choices[0]
            .message
            .tool_calls
            .as_ref()
            .is_none_or(Vec::is_empty)
    }

    pub fn content(self) -> Option<String> {
        self.message().content
    }

    pub fn message(self) -> ResponseMessage {
        self.choices.into_iter().next().unwrap().message
    }
}

impl ResponseMessage {
    pub fn execute(&self) -> Result<Vec<Message>, ResponseError> {
        let mut messages = Vec::new();
        if let Some(tool_calls) = &self.tool_calls {
            for tool_call in tool_calls {
                messages.push(tool_call.execute()?);
            }
        }
        Ok(messages)
    }
}

impl ToolCall {
    pub fn parse_arguments(&self) -> HashMap<String, String> {
        serde_json::from_str(&self.function.arguments).unwrap()
    }

    pub fn execute(&self) -> Result<Message, ResponseError> {
        let args = self.parse_arguments();
        let result = match self.function.name {
            ToolName::Read => {
                let file_path = args.get("file_path").unwrap();
                let content = read_to_string(file_path)?;
                Message::Tool(ToolMessage::new(self.id.clone(), Some(content)))
            }
            ToolName::Write => {
                let file_path = args.get("file_path").unwrap();
                let content = args.get("content").unwrap();
                std::fs::write(file_path, content)?;
                Message::Tool(ToolMessage::new(self.id.clone(), None))
            }
        };
        Ok(result)
    }
}
