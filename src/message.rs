#[derive(Clone)]
pub struct Message {
    content: String,
    is_user: bool,
}

impl Message {
    pub fn new(content: String, is_user: bool) -> Self {
        Self { content, is_user }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_user(&self) -> bool {
        self.is_user
    }
}