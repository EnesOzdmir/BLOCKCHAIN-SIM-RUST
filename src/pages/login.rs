use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Element, Length,
};
use iced_aw::tab_bar::TabLabel;
use mini_redis::client;
use tokio::net::TcpListener;
use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
    Block,

}

#[derive(Default)]
pub struct LoginTab {
    username: String,
    password: String,
}

impl LoginTab {
    pub fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::UsernameChanged(value) => self.username = value,
            LoginMessage::PasswordChanged(value) => self.password = value,
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
            }
            LoginMessage::LoginPressed => {

                futures::executor::block_on(async { let mut client = client::connect("127.0.0.1:6379").await;
                    client.expect("REASON").set(&self.username,self.password.clone().into()).await;});
                dbg!(&self.username);
                dbg!(self.password.clone());
                
                println!("Logged in successfully")
            }
            LoginMessage::Block => {}
        }
    }
}

impl Tab for LoginTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())

        TabLabel::IconText(Icon::Heart.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_x(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextInput::new("Pls now index", &self.username)
                        .on_input(LoginMessage::UsernameChanged)
                        .padding(10)
                        .size(32),
                )
                .push(
                    TextInput::new("Pls prev hash", &self.password)
                        .on_input(LoginMessage::PasswordChanged)
                        .padding(10)
                        .size(32)
                        .secure(true),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(Text::new("Clear").align_x(Horizontal::Center))
                                .width(Length::Fill)
                                .on_press(LoginMessage::ClearPressed),
                        )
                        .push(
                            Button::new(Text::new("Login").align_x(Horizontal::Center))
                                .width(Length::Fill)
                                .on_press(LoginMessage::LoginPressed),
                        ),
                ),
        )
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();

        content.map(Message::Login)
    }
}
