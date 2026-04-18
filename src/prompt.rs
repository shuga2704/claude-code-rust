use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptMessage {
    role: String,
    content: String,
}

pub struct Prompt {
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl From<serde_json::Error> for ParseError {
    fn from(error: serde_json::Error) -> ParseError {
        ParseError {
            message: error.to_string(),
        }
    }
}

impl PromptMessage {
    pub fn initial(prompt: String) -> Self {
        PromptMessage {
            role: "user".to_string(),
            content: prompt,
        }
    }
}

impl Prompt {
    pub fn new() -> Self {
        Prompt {
            messages: Vec::new(),
        }
    }

    pub fn to_json(&self) -> Result<serde_json::Value, ParseError> {
        let parsed_messages = self
            .messages
            .iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(serde_json::Value::Array(parsed_messages))
    }
}

impl Deref for Prompt {
    type Target = Vec<Message>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for Prompt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}
