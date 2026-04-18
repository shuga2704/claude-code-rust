use serde::{Deserialize, Serialize};

use crate::{prompt::PromptMessage, response::ResponseMessage, tool::ToolMessage};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    Prompt(PromptMessage),
    Tool(ToolMessage),
    Response(ResponseMessage),
}
