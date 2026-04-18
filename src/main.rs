use async_openai::{config::OpenAIConfig, Client};
use clap::Parser;
use dotenvy::dotenv;
use serde_json::{from_str, json, to_value, Value};
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();

    let is_local = std::env::var("LOCAL")
        .map(|local| local == "true")
        .unwrap_or(false);

    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let mut messages = vec![json!({"role": "user", "content": args.prompt})];

    loop {
        let response: Value = client
            .chat()
            .create_byot(json!({
                "messages": messages,
                "model": env::var("OPENROUTER_MODEL").unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string()),
                "tools": [{
                    "type": "function",
                    "function": {
                        "name": "Read",
                        "description": "Read and return the contents of the file",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                    "type": "string",
                                    "description": "The path to the file to read"
                                }
                            },
                            "required": ["file_path"],
                        }
                    }
                }]
            }))
            .await?;

        let message = &response["choices"][0]["message"];
        messages.push(to_value(&message)?);

        if let Some(tool_calls) = message["tool_calls"].as_array() {
            let tool_call = &tool_calls[0];
            let name = tool_call["function"]["name"].as_str().unwrap();

            let args: Value =
                from_str(tool_call["function"]["arguments"].as_str().unwrap()).unwrap();

            if name == "Read" {
                let file_path = args["file_path"].as_str().unwrap();
                let content = std::fs::read_to_string(file_path)?;

                messages.push(
                    json!({"role": "tool", "tool_call_id": tool_call["id"], "content": content}),
                );
            }
        } else if let Some(content) = message["content"].as_str() {
            println!("{}", content);
            break;
        }
    }

    Ok(())
}
