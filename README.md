# Rust Chatbot UI

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![ChatGPT](https://img.shields.io/badge/chatGPT-74aa9c?style=for-the-badge&logo=openai&logoColor=white)
![Claude](https://img.shields.io/badge/Claude-7A13C1?style=for-the-badge&logo=anthropic&logoColor=white)
![Meta](https://img.shields.io/badge/Meta_AI-0467DF?style=for-the-badge&logo=meta&logoColor=white)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The Rust Chatbot UI supports multiple AI providers including GPT, Claude, and Fireworks, with the ability to switch between models mid-conversation. It features an easy-to-use interface with customizable themes and comprehensive chat history management. The application allows for real-time model adaptation during chats and includes export functionality for saving conversations. Built in Rust, it offers efficient performance while maintaining context across different AI models and providers.

![image](https://github.com/user-attachments/assets/c9f79bab-606f-41f1-9650-ff1946a9b4ee)

## Features

- **Multi-Provider Support**: Integrate with various AI providers including GPT, Claude, and Fireworks.
- **Dynamic Model Switching**: Seamlessly change AI models mid-conversation without losing context.
- **Intuitive Interface**: Easy-to-use design for smooth interactions.
- **Customizable Themes**: Personalize your chat environment with different visual themes.
- **Chat History Management**: Efficiently organize and access your past conversations.
- **Export Functionality**: Save and share your chat sessions with ease.
- **Stop Generation**: Ability to stop the model's response generation at any time.
- **Secure API Key Storage**: API keys are securely stored in system credential managers, not in plain text files.
- **AI-Generated Chat Names**: Automatically generate relevant names for your chat sessions using AI.

## Quick Start

1. Download the latest binary for your system (Windows or Linux) from the [Releases](https://github.com/ddxfish/rust_chatbot_ui/releases) page.

2. If you prefer to build from source:
   ```
   git clone https://github.com/ddxfish/rust_chatbot_ui
   cd rust-chatbot-ui
   cargo build --release
   ```
   The executable will be in `target/release/rust_chatbot_ui`.

3. Run the application, enter your API keys in the Settings panel, and start chatting!

## Configuration

API keys for different providers can be entered in the Settings panel within the application.

## License

This project is licensed under the Apache License 2.0. In simple terms:

- You're free to use, modify, and distribute this software.
- Please include a link to this GitHub project when you use or redistribute it.
- The full license text can be found in the [LICENSE](LICENSE) file.

For more details, see: [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)

## Acknowledgments

- [egui](https://github.com/emilk/egui) for the GUI framework
- [Claude](https://www.anthropic.com) chatbot for assistance in development
- [Fireworks.ai](https://fireworks.ai/) for AI services
- [Rust programming language](https://www.rust-lang.org/) for enabling efficient and safe development