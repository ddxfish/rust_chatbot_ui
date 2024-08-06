#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Send,
    Received(Result<String, String>),
}