use std::process::Command;
use iced::{alignment::{Horizontal, Vertical}, widget::{Button, Column, Container, Row, Text, TextInput}, Alignment, Center, Element, Fill, Length};
use iced::widget::{button, column, container, value};
use iced_aw::tab_bar::TabLabel;
use mini_redis::client;
use mini_redis::client::Client;
use crate::{blockchain, Icon, Message, Tab};
use crate::blockchain::block::Block;
use crate::pages::login::LoginMessage;

#[derive(Debug, Clone)]
pub enum BlockMessage {
    ButtonPressed,
    NewData(String),
    NewPrewHash(String),
    NewDevice,

}

#[derive(Default)]
pub struct BlockTab {
    index:u32,
    hash:String,
    prevhash:String,
    nonce:u32,
    data:String,
}

impl BlockTab {
    pub fn update(&mut self, message: BlockMessage) {
        match message {
            BlockMessage::ButtonPressed  => {

                 let mut lambo = async {
                    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
                    let result = client.get("a").await.unwrap();



                    match result {
                        Some(value) => {
                            match str::from_utf8(&value) {
                                Ok(my_string) => {
                                    println!("fukkk");
                                    my_string.to_string()
                                }
                                Err(e) => {
                                    format!("UTF-8 çevirme hatası: {}", e).to_string()
                                }
                            }
                        }
                        None => {
                            "hell".to_string()
                        }
                    }
                };

                let prevhash_value = futures::executor::block_on(lambo);







                let create = |data: String| -> Block{ blockchain::block::Block::block_create(data) };
                let mut a = create(self.data.clone());
                a = a.mine(prevhash_value.clone());
                a.prevhash = prevhash_value;
                self.index = a.index;
                self.hash = a.hash;
                self.prevhash = a.prevhash;
                self.nonce = a.nonce;
                self.data = a.data;

            },
            BlockMessage::NewData(data) => {
                self.data=data;
            }
            BlockMessage::NewPrewHash(NewPrewHash) => {
                self.prevhash=NewPrewHash;
            }
            BlockMessage::NewDevice =>  {
                let mut command= Command::new("./target/debug/untitled2");
                match command.status() {
                    Ok(status) => println!("durum kodu: {}", status),
                    Err(e) => eprintln!("Alt program başlatılamadı: {}", e),
                }
            }
        }
    }
}

impl Tab for BlockTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Cook Mineee!")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())

        TabLabel::IconText(Icon::Heart.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {




        let block_element: Element<'_, BlockMessage> = Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(Column::new()
                                  .push(Text::new("Index")).align_x(Center)
                                  .push(Text::new(self.index)).width(Fill).align_x(Center) // & kaldırıldı
                        )
                )
                .push(
                    Row::new()
                        .push(Column::new()
                            .push(Text::new("Data")).align_x(Center)
                            .push(
                                TextInput::new("", &self.data)
                                    .on_input(BlockMessage::NewData)
                                    .padding(10)
                                    .size(16)
                                    .width(350)
                                    .align_x(Center)
                            ).align_x(Center).width(Length::Fill)
                        ).width(Length::Fill).align_y(Center)
                )
                .push(
                    Row::new()
                        .push(Column::new()
                                  .push(Text::new("Nonce")).align_x(Center).width(Fill)
                                  .push(Text::new(self.nonce.to_string())) // & kaldırıldı
                        )
                ).width(Length::Fill).align_x(Center)
                .push(
                    Row::new()
                        .push(Column::new()
                                  .push(Text::new("Hash")).width(Fill).align_x(Center)
                                  .push(Text::new(self.hash.to_string())).width(350).align_x(Center)// & kaldırıldı
                        )
                ).width(Length::Fill).align_x(Center)
                .push(
                    Row::new()
                        .push(Column::new()
                                  .push(Text::new("Previous Hash")).width(Fill).align_x(Center)
                                  .push(Text::new(self.prevhash.to_string())) // & kaldırıldı
                        )
                ).width(Length::Fill).align_x(Center)
                .push(Button::new("Mine!!").on_press(BlockMessage::ButtonPressed)).spacing(20)
                .push(Button::new("New Device!!").on_press(BlockMessage::NewDevice))
        )
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        block_element.map(Message::Block)
    }
}
