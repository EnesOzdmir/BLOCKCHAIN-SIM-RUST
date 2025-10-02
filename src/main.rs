mod blockchain;
mod pages;

use pages::*;
use std::process::Command;
use iced::{window, Center, Element, Fill, Font};
use iced::widget::{Container,Column};
use crate::blockchain::block::Block;

use std::env;
use iced_aw::{TabBarPosition, TabLabel, Tabs};
use iced_aw::style::tab_bar;
use mini_redis::{client, Result};
use mini_redis::client::Client;
use tokio::net::ToSocketAddrs;
use crate::pages::login::LoginTab;
use crate::pages::Settings::*;
use crate::pages::block::*;

const ICON_BYTES: &[u8] = include_bytes!("../../BLOCKCHAIN-SIM-RUST/src/fonts/icons.ttf");
const ICON: Font = Font::with_name("icons");

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}


#[tokio::main]
async fn main() -> iced::Result {


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
        .font(ICON_BYTES)
        .run()
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
    client::connect(addr).await
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(TabId),
    Login(login::LoginMessage),
    Settings(SettingsMessage),
    TabClosed(TabId),
    Block(BlockMessage)
}

#[derive(Default)]
struct MyApp {
    index:u32,
    hash:String,
    prevhash:String,
    nonce:u32,
    data:String,
    active_tab: TabId,
    login_tab: LoginTab,
    settings_tab: SettingsTab,
    block_tab: BlockTab}


#[derive(Clone, PartialEq, Eq, Debug, Default)]
enum TabId {
    #[default]
    Login,
    Settings,
    Block,
}

impl MyApp {
    fn update(&mut self, message: Message, ) {
        match message {
            Message::TabClosed(id) => println!("Tab {:?} event hit", id),
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) =>  self.login_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
            Message::Block(value) => self.block_tab.update(value)

        }
    }

    fn view(&self) -> iced::Element<Message> {



     /*   let mut res = Ok(None);
        async {
            let mut client = connect("127.0.0.1:6379").await;
            res = client.expect("REASON").get(&index.to_string()).await;
        };

        let a = match res {
            Ok(Some(res)) => res,
            _ =>,
        };

        let s = match str::from_utf8(&*a) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }; */





   /* let a = container(
        column![
            row![column![text("Index").align_x(Center), text(&self.index).align_x(Center)].align_x(Center).padding(5)],
            row![column![text("Data").align_x(Center), text_input("",&self.data).on_input(Message::NewData).align_x(Center).width(250)].align_x(Center).padding(5)],
            row![column![text("Nonce").align_x(Center), text(&self.nonce).align_x(Center)].align_x(Center).padding(5)],
            row![column![text("Hash").align_x(Center), text(&self.hash).align_x(Center).width(250)].align_x(Center).padding(5)],
            row![column![text("PrevHash").align_x(Center), text_input("",&self.prevhash).on_input(Message::NewPrewHash).align_x(Center).width(250)].align_x(Center).padding(5)],
          column![button(text("Mineee!")).padding(5).on_press(Message::ButtonPressed),
           container(button(text("New Device")).padding(5).on_press(unsafe{Message::NewDevice})).padding(10)].align_x(Center).padding(10),
    ].align_x(Center)).align_y(Center).align_x(Center).width(Fill).height(Fill).into();

        a */
        Tabs::new(Message::TabSelected)
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .on_close(Message::TabClosed)
            .push(
                TabId::Login,
                self.login_tab.tab_label(),
                self.login_tab.view(),
            )
            .push(
                TabId::Block,
                self.block_tab.tab_label(),
                self.block_tab.view(),
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_style(tab_bar::primary)
            .tab_bar_position(TabBarPosition::Top)
            .into()
    }

}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .align_y(Center)
            .padding(10)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}



