mod cli;
mod client;

use clap::Parser;
use cli::{Cli, Commands, AlertCommand, ListCommand};
use client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Optionally load from .env file
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();
    let client = Client::new(cli.api_key.clone());

    let result = match &cli.command {
        Commands::Status => {
            let res = client.get("/status").await?;
            res
        }
        Commands::Price { ticker, history } => {
            let path = if *history {
                format!("/price/{}/history", ticker)
            } else {
                format!("/price/{}", ticker)
            };
            client.get(&path).await?
        }
        Commands::Indicators { ticker } => {
            client.get(&format!("/indicators/{}", ticker)).await?
        }
        Commands::Account => {
            client.get("/account").await?
        }
        Commands::Alert { action } => match action {
            AlertCommand::Ls { status, list } => {
                let mut params = Vec::new();
                if let Some(s) = status {
                    params.push(format!("status={}", s));
                }
                if let Some(l) = list {
                    params.push(format!("listId={}", l));
                }
                
                let path = if params.is_empty() {
                    "/alerts".to_string()
                } else {
                    format!("/alerts?{}", params.join("&"))
                };
                client.get(&path).await?
            }
            AlertCommand::Get { id } => {
                client.get(&format!("/alerts/{}", id)).await?
            }
            AlertCommand::Create { prompt, list, note, pin, color, full } => {
                let mut body = serde_json::json!({ "prompt": prompt });
                
                if let Some(obj) = body.as_object_mut() {
                    if let Some(l) = list {
                        obj.insert("listId".to_string(), serde_json::json!(l));
                    }
                    if let Some(n) = note {
                        obj.insert("note".to_string(), serde_json::json!(n));
                    }
                    if *pin {
                        obj.insert("isPinned".to_string(), serde_json::json!(true));
                    }
                    if let Some(c) = color {
                        obj.insert("color".to_string(), serde_json::json!(c));
                    }
                }

                let path = if *full {
                    "/alerts?full=true"
                } else {
                    "/alerts"
                };
                client.post(path, &body).await?
            }
            AlertCommand::Edit { id, list, note, pin, unpin, color, clear_color } => {
                let mut body = serde_json::json!({});
                if let Some(obj) = body.as_object_mut() {
                    if let Some(l) = list {
                        obj.insert("listId".to_string(), serde_json::json!(l));
                    }
                    if let Some(n) = note {
                        obj.insert("note".to_string(), serde_json::json!(n));
                    }
                    if *pin {
                        obj.insert("isPinned".to_string(), serde_json::json!(true));
                    }
                    if *unpin {
                        obj.insert("isPinned".to_string(), serde_json::json!(false));
                    }
                    if let Some(c) = color {
                        obj.insert("color".to_string(), serde_json::json!(c));
                    }
                    if *clear_color {
                        obj.insert("color".to_string(), serde_json::Value::Null);
                    }
                }
                client.patch(&format!("/alerts/{}", id), &body).await?
            }
            AlertCommand::Archive { id } => {
                client.delete(&format!("/alerts/{}", id)).await?
            }
        }
        Commands::List { action } => match action {
            ListCommand::Ls => {
                client.get("/lists").await?
            }
        }
    };

    if cli.output == "json" {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        // Human readable output
        // Based on the data returned, we'd implement comfy-table matching here.
        // For brevity in scaffold, default to pretty JSON if table formatter not implemented for that response.
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
