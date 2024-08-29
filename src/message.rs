#[derive(Clone)]
pub struct Message {
    content: String,
    is_user: bool,
    model: Option<String>,
}

impl Message {
    pub fn new(content: String, is_user: bool, model: Option<String>) -> Self {
        Self { content, is_user, model }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_user(&self) -> bool {
        self.is_user
    }

    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }
}