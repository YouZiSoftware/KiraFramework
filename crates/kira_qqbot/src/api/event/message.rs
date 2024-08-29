use rust_i18n::t;
use serde::{Deserialize, Serialize};
use kira_framework::network::message_chain::MessageChain;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotEvent;
use kira_framework_proc::onebot_event_type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnonymousMessage {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(message = "group")]
pub struct GroupMessage {
    pub sub_type: String,
    pub message_id: i32,
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<AnonymousMessage>,
    pub message: MessageChain,
    pub raw_message: String,
    pub font: i32,
    pub sender: Sender,
}

impl KiraPrettyDebug for GroupMessage {
    fn pretty_debug(&self) -> String {
        let group_id = self.group_id;
        let sender = self.sender.clone();
        let name = sender.nickname.unwrap();
        let qq = sender.user_id.unwrap();
        let message = self.raw_message.clone();
        format!("{}", t!("console.recv.group", group = group_id, id = self.message_id, name = name, qq = qq, message = message))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(message = "private")]
pub struct PrivateMessage {
    pub sub_type: String,
    pub message_id: i32,
    pub user_id: i64,
    pub message: MessageChain,
    pub raw_message: String,
    pub font: i32,
    pub sender: Sender,
}

impl KiraPrettyDebug for PrivateMessage {
    fn pretty_debug(&self) -> String {
        let sender = self.sender.clone();
        let name = sender.nickname.unwrap();
        let qq = self.user_id;
        let message = self.raw_message.clone();
        format!("{}", t!("console.recv.private", name = name, qq = qq, message = message))
    }
}