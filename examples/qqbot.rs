use std::time::Duration;
use bevy_app::{Plugin, Update};
use bevy_ecs::event::EventReader;
use bevy_ecs::system::Res;
use kira_framework::{BotApp, kira_async, kira_recv, messages};
use kira_framework::network::connect::OneBotConnect;
use kira_framework::network::connect::reverse::OneBotReverseWebSocket;
use kira_framework::network::events::OneBotEventReceiver;
use kira_qqbot::api::event::message::GroupMessage;
use kira_qqbot::api::event::OneBotEvents;
use kira_qqbot::{at, text};
use kira_qqbot::connect::KiraQQBotConnect;

fn main() {
    BotApp::new()
        .onebot_connect(OneBotConnect::new(
            OneBotReverseWebSocket::new(
                "127.0.0.1:8080",//your ip and port
                Some("access token"),//your access token
                Duration::from_secs(5)//reconnect interval
            )
        ))
        .add_lib_plugins("./plugins").unwrap()//load dylib plugins from "./plugins" dir
        .add_lib_plugins("./my_plugin.dll").unwrap()//load dylib plugin
        .add_plugins(MyQQBot)
        .run::<OneBotEvents>();
}

pub struct MyQQBot;

impl MyQQBot {
    //receive group message from QQ
    fn recv_group_message(mut receiver: EventReader<OneBotEventReceiver<GroupMessage>>, connect: Res<OneBotConnect>) {
        //use macro to receive event
        kira_recv!(receiver(let event) => {
            kira_async!("recv_event" => {
                let message = event.message;
                let _raw_message = message.as_persistent_string();//you can get raw message from MessageChain
                //or you can get raw message from event
                let raw_message = event.raw_message;
                let connect = KiraQQBotConnect::new(connect.clone());//convert OneBotConnect to KiraQQBotConnect
                if raw_message == "test" {
                    let message_id = connect.send_group_message(
                        event.group_id,
                        messages![
                            at!(event.sender.user_id.unwrap()),
                            text!("hello world!")
                        ],//use macro to easily create MessageChain
                        false
                    ).await.unwrap();
                }
            });
        });
    }
}

impl Plugin for MyQQBot {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, Self::recv_group_message);
    }
}