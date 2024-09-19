use super::chat_core::Chat;
use super::file_operations;
use crate::message::Message;

impl Chat {
    pub fn check_ui_updates(&self) -> Option<(String, bool)> {
        self.ui_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn check_name_updates(&self) -> Option<String> {
        self.name_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn check_error_updates(&self) -> Option<String> {
        self.error_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn create_new_chat(&self) -> Result<(), std::io::Error> {
        self.messages.lock().unwrap().clear();
        *self.needs_naming.lock().unwrap() = true;
        let new_file = self.history_manager.lock().unwrap().create_new_chat()?;
        self.load_chat(&new_file)?;
        self.set_has_updates();
        self.clear_syntax_cache();
        Ok(())
    }

    pub fn load_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        *self.needs_naming.lock().unwrap() = false;
        self.history_manager.lock().unwrap().load_chat(file_name, &mut self.messages.lock().unwrap())?;
        self.set_has_updates();
        self.clear_syntax_cache();
        Ok(())
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.history_manager.lock().unwrap().get_current_file()
    }

    pub fn delete_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        let mut history_manager = self.history_manager.lock().unwrap();
        if let Some(new_file) = history_manager.delete_chat(file_name)? {
            drop(history_manager);
            self.load_chat(&new_file)?;
        } else if history_manager.get_current_file().is_none() {
            if let Some(first_file) = history_manager.get_history_files().first() {
                drop(history_manager);
                self.load_chat(first_file)?;
            }
        }
        self.set_has_updates();
        self.clear_syntax_cache();
        Ok(())
    }

    pub fn export_chat(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        file_operations::export_chat(path, &self.messages.lock().unwrap())
    }

    pub fn rename_current_chat(&self, new_name: &str) -> Result<(), std::io::Error> {
        self.history_manager.lock().unwrap().rename_current_chat(new_name)?;
        self.set_has_updates();
        self.clear_syntax_cache();
        Ok(())
    }

    pub fn get_current_model(&self) -> String {
        self.current_model.lock().unwrap().clone()
    }

    pub fn set_current_model(&self, model: &str) {
        *self.current_model.lock().unwrap() = model.to_string();
        self.set_has_updates();
    }

    pub fn load_most_recent_or_create_new(&self) -> Result<(), std::io::Error> {
        let history = self.history_manager.lock().unwrap();
        let files = history.get_history_files();
        drop(history);

        if let Some(most_recent) = files.first() {
            self.load_chat(most_recent)
        } else {
            self.create_new_chat()
        }
    }

    pub fn has_updates(&self) -> bool {
        let mut has_updates = self.has_updates.lock().unwrap();
        let updates = *has_updates;
        *has_updates = false;
        updates
    }

    pub(crate) fn set_has_updates(&self) {
        *self.has_updates.lock().unwrap() = true;
    }

    pub fn add_message(&self, content: String, is_user: bool) {
        let model = if is_user { None } else { Some(self.get_current_model()) };
        let message = Message::new(content.clone(), is_user, model.clone());
        self.messages.lock().unwrap().push(message);
        if let Err(e) = self.history_manager.lock().unwrap().append_message(&content, is_user, model.as_deref()) {
            eprintln!("Failed to append message to history: {}", e);
        }

        if !is_user && *self.needs_naming.lock().unwrap() {
            self.generate_chat_name();
        }
        self.set_has_updates();
        self.clear_syntax_cache();
    }
}