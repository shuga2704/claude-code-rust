mod chat;
mod message;
mod prompt;
mod response;
mod tool;

use crate::chat::Chat;
use async_openai::config::OpenAIConfig;
use clap::Parser;
use dotenvy::dotenv;
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

    let mut chat = Chat::default(config);

    match chat.send(args.prompt).await {
        Ok(Some(response)) => {
            println!("{response}");
        }
        Err(e) => {
            eprintln!("Error sending prompt: {}", e.message);
        }
        _ => {
            eprintln!("No response received");
        }
    };

    Ok(())
}
