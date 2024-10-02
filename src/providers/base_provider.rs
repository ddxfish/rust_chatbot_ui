use crate::providers::ProviderError;
use crate::app::ProfileType;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc;
use std::fmt;

pub struct BaseProvider {
    pub client: Client,
    pub api_key: String,
    pub top_p: f32,
    pub top_k: u32,
    pub repetition_penalty: f32,
    pub creativity: f32,
}

impl BaseProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            top_p: 0.9,
            top_k: 50,
            repetition_penalty: 0.2,
            creativity: 0.8,
        }
    }

    pub fn update_profile(&mut self, profile: ProfileType) {
        match profile {
            ProfileType::Coder => {
                self.top_p = 0.85;
                self.top_k = 40;
                self.repetition_penalty = 0.04;
                self.creativity = 0.4;
            },
            ProfileType::Normal => {
                self.top_p = 0.9;
                self.top_k = 50;
                self.repetition_penalty = 0.15;
                self.creativity = 0.8;
            },
            ProfileType::Creative => {
                self.top_p = 0.95;
                self.top_k = 80;
                self.repetition_penalty = 0.4;
                self.creativity = 1.4;
            },
        }
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_parameters(&self) -> (f32, u32, f32, f32) {
        (self.top_p, self.top_k, self.repetition_penalty, self.creativity)
    }
}


pub trait ProviderTrait: fmt::Display + Send + Sync {
    fn name(&self) -> &'static str;
    fn models(&self) -> Vec<(&'static str, usize)>;
    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError>;
    fn set_current_model(&self, model: String);
    fn update_profile(&self, profile: ProfileType);
    fn get_parameters(&self) -> (f32, u32, f32, f32);
}
