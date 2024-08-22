use super::{Provider, ProviderError};
use std::fmt;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct Claude {
    client: Client,
    api_key: String,
}

impl Claude {
    pub fn new(api_key: String) -> Self {
        println!("Debug: Creating new Claude instance");
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for Claude {
    fn name(&self) -> &'static str {
        "Claude"
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "claude-3-5-sonnet-20240620",
            "claude-3-opus-20240229",
            "claude-3-haiku-20240307",
        ]
    }

    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        println!("Debug: Starting Claude stream_response");
        let (tx, rx) = mpsc::channel(1024);
        let client = self.client.clone();
        let api_key = self.api_key.clone();

        tokio::spawn(async move {
            println!("Debug: Sending request to Claude API");
            let response = match client
                .post("https://api.anthropic.com/v1/messages")
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .header("x-api-key", api_key)
                .json(&json!({
                    "model": "claude-3-5-sonnet-20240620",
                    "messages": messages,
                    "max_tokens": 1024,
                    "stream": true
                }))
                .send()
                .await {
                    Ok(res) => {
                        println!("Debug: Successfully sent request to Claude API");
                        res
                    },
                    Err(e) => {
                        eprintln!("Error sending request to Claude API: {:?}", e);
                        return;
                    }
                };

            println!("Debug: Starting to process Claude API response stream");
            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                match item {
                    Ok(chunk) => {
                        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                            for line in text.lines() {
                                if line.starts_with("data: ") {
                                    let data = &line[6..];
                                    if data == "[DONE]" {
                                        println!("Debug: Claude stream completed");
                                        return;
                                    }

                                    match serde_json::from_str::<Value>(data) {
                                        Ok(json) => {
                                            if let Some(content) = json["delta"]["text"].as_str() {
                                                println!("Debug: Received content chunk from Claude: {}", content);
                                                if tx.send(content.to_string()).await.is_err() {
                                                    eprintln!("Error sending chunk through channel");
                                                    return;
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("Error parsing JSON from Claude: {:?}", e),
                                    }
                                }
                            }
                        } else {
                            eprintln!("Error converting Claude response chunk to UTF-8");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving chunk from Claude: {:?}", e);
                    }
                }
            }
            println!("Debug: Finished processing Claude API response stream");
        });

        println!("Debug: Returning Claude response receiver");
        Ok(rx)
    }
}

impl fmt::Display for Claude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Claude")
    }
}