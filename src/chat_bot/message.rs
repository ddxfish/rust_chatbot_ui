#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Send,
    Received(Result<String, String>),
    InputSubmit, // New variant for Enter key press
}