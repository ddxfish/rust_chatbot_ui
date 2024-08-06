use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};

use super::{ChatBot, Message};

pub fn view(chat_bot: &ChatBot) -> Element<Message> {
    let input = text_input("Type a message...", &chat_bot.input)
        .on_input(Message::InputChanged)
        .on_submit(Message::InputSubmit) // Add this line to handle Enter key press
        .padding(10);

    let send_button = button("Send").on_press(Message::Send);

    let input_row = row![input, send_button].spacing(10).align_items(Alignment::Center);

    let messages: Element<_> = chat_bot
        .messages
        .iter()
        .fold(column![].spacing(10), |column, msg| {
            column.push(text(msg))
        })
        .into();

    let content = column![
        scrollable(container(messages).width(Length::Fill).padding(20))
            .height(Length::Fill),
        input_row
    ]
    .spacing(20)
    .padding(20)
    .align_items(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}