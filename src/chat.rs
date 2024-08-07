use crate::message::Message;
use crate::chatbot::Chatbot;

pub struct Chat {
    messages: Vec<Message>,
    chatbot: Chatbot,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            chatbot: Chatbot::new(),
        }
    }

    pub fn add_message(&mut self, content: String, is_user: bool) {
        let message = Message::new(content, is_user);
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn process_input(&mut self, input: String) {
        self.add_message(input.clone(), true);
        let response = self.chatbot.generate_response(&self.messages);
        self.add_message(response, false);
    }
}