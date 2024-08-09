use crate::message::Message;

pub struct Chatbot;

impl Chatbot {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_response(&self, messages: &Vec<Message>) -> String {
        // This is a placeholder implementation. Replace this with your actual chatbot logic.
        let context = messages.iter()
            .map(|m| m.content())
            .collect::<Vec<&str>>()
            .join(" ");

        format!("I received your messagesdasdafasa asd fDASF fsfasf sdaafd  sd fasdf dasf adfasdfsd fasd d fdasfasdfasdf.  fdasfasdfasdf.  fdasfasdfasdf.  fdasfasdfasdf.  fdasfasdfasdf. Context length: {} characters.", context.len())
    }
}