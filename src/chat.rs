use async_openai::{config::OpenAIConfig, Client};

use crate::{
    message::Message,
    prompt::{ParseError, Prompt, PromptMessage},
    response::{ResponseError, Responses},
    tool::AgentTool,
};

pub struct ChatError {
    pub message: String,
}

impl From<async_openai::error::OpenAIError> for ChatError {
    fn from(error: async_openai::error::OpenAIError) -> ChatError {
        ChatError {
            message: error.to_string(),
        }
    }
}

impl From<ResponseError> for ChatError {
    fn from(error: ResponseError) -> ChatError {
        ChatError {
            message: error.message,
        }
    }
}

impl From<ParseError> for ChatError {
    fn from(error: ParseError) -> ChatError {
        ChatError {
            message: error.message,
        }
    }
}

pub struct Chat {
    client: Client<OpenAIConfig>,
    model: String,
    tools: Vec<AgentTool>,
    max_tokens: u32,
    history: Prompt,
}

impl Chat {
    pub fn default(config: OpenAIConfig) -> Self {
        Chat {
            client: Client::with_config(config),
            model: "anthropic/claude-haiku-4.5".to_string(),
            tools: vec![AgentTool::read(), AgentTool::write()],
            max_tokens: 400,
            history: Prompt::new(),
        }
    }

    async fn get_response(&self) -> Result<Responses, ChatError> {
        let response = self
            .client
            .chat()
            .create_byot(serde_json::json!({
                "messages": self.history.to_json()?,
                "model": self.model,
                "tools": self.tools,
                "max_tokens": self.max_tokens,
            }))
            .await?;

        Ok(Responses::from_value(response)?)
    }

    pub async fn send(&mut self, prompt: String) -> Result<Option<String>, ChatError> {
        self.history
            .push(Message::Prompt(PromptMessage::initial(prompt)));

        loop {
            let response = self.get_response().await?;

            if response.is_finished() {
                return Ok(response.content());
            }

            let mut result = Vec::new();
            for message in response.execute()? {
                result.push(message);
            }

            self.history.push(Message::Response(response.message()));
            self.history.extend(result);
        }
    }
}
