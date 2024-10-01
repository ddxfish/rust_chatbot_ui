use super::{ProviderError, ProviderTrait};
use std::fmt;
use serde_json::Value;
use tokio::sync::mpsc;
use crate::app::ProfileType;

pub struct None;

impl None {
    pub fn new() -> Self {
        Self
    }
}

impl ProviderTrait for None {
    fn name(&self) -> &'static str {
        "Select a provider"
    }

    fn models(&self) -> Vec<(&'static str, usize)> {
        vec![("Then select model", 0)]
    }

    fn stream_response(&self, _messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1);
        tokio::task::spawn(async move {
            let _ = tx.send("API key goes in Settings. Then select a provider and model.".to_string()).await;
        });
        Ok(rx)
    }

    fn set_current_model(&self, _model: String) {
        // Do nothing for None provider
    }

    fn update_profile(&self, _profile: ProfileType) {
        // Do nothing for None provider
    }
    fn get_parameters(&self) -> (f32, u32, f32, f32) {
        (1.0, 1, 1.0, 1.0)
    }
}

impl fmt::Display for None {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "None")
    }
}