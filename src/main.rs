mod blockchain;

use std::process::Command;
use iced::overlay::menu::State;
use iced::{window, Center, Element, Fill, Theme};
use iced::widget::{container, Container, column, button, text, row, center, text_input};
use crate::blockchain::*;
use crate::blockchain::block::Block;

fn main() -> iced::Result {
    iced::application("Main Controller",MyApp::update,MyApp::view)
        .centered()
        .window(window::Settings {
            icon: Some(
                window::icon::from_file_data(
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png")),
                    None,
                )
                    .expect("icon file should be reachable and in ICO file format"),
            ),
            ..Default::default()
        })
        .theme(|_s| iced::theme::Theme::CatppuccinFrappe)
        .window_size(iced::Size::new(450.0, 600.0))
        .run()
}


#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    NewData(String),
    NewPrewHash(String),
    NewDevice
}

#[derive(Default)]
struct MyApp {
    index:u32,
    hash:String,
    prevhash:String,
    nonce:u32,
    data:String,}

impl MyApp {
    fn update(&mut self, message: Message) {
        let index=1;
        match message {
            Message::ButtonPressed  => {
                let create = |index:u32,data:String| -> Block{ blockchain::block::Block::block_create(index, data) };
                let mut a = create(1, self.data.clone());
                a = a.mine();
                self.index =a.index;
                self.hash = a.hash;
                self.prevhash = a.prevhash;
                self.nonce=a.nonce;
                self.data=self.data.clone();;
            },
            Message::NewData(data) => {
                self.data=data;
            }
            Message::NewPrewHash(NewPrewHash) => {
                self.prevhash=NewPrewHash;
            }
            Message::NewDevice => {
               let mut command= Command::new("./target/debug/untitled2");
                match command.status() {
                    Ok(status) => println!("durum kodu: {}", status),
                    Err(e) => eprintln!("Alt program başlatılamadı: {}", e),
                }
            }

        }
    }

    fn view(&self) -> iced::Element<Message> {
    container(
        column![
            row![column![text("Index").align_x(Center), text(&self.index).align_x(Center)].align_x(Center).padding(5)],
            row![column![text("Data").align_x(Center), text_input("",&self.data).on_input(Message::NewData).align_x(Center).width(250)].align_x(Center).padding(5)],
            row![column![text("Nonce").align_x(Center), text(&self.nonce).align_x(Center)].align_x(Center).padding(5)],
            row![column![text("Hash").align_x(Center), text(&self.hash).align_x(Center).width(250)].align_x(Center).padding(5)],
            row![column![text("PrevHash").align_x(Center), text_input("",&self.prevhash).on_input(Message::NewPrewHash).align_x(Center).width(250)].align_x(Center).padding(5)],
          column![button(text("Mineee!")).padding(5).on_press(Message::ButtonPressed),
           container(button(text("New Device")).padding(5).on_press(Message::NewDevice)).padding(10)].align_x(Center).padding(10),
    ].align_x(Center)).align_y(Center).align_x(Center).width(Fill).height(Fill).into()
    }

}


