use rust_i18n::t;
use serde::{Deserialize, Serialize};
use kira_framework::network::message_chain::MessageChainType;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotEvent;
use kira_framework_proc::onebot_event_type;
use crate::api::anonymous::AnonymousMessage;
use crate::api::sender::Sender;

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(message = "group")]
pub struct GroupMessage {
    pub sub_type: String,
    pub message_id: i32,
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<AnonymousMessage>,
    pub message: MessageChainType,
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
        format!("{}", t!("console.event.message.group", group = group_id, id = self.message_id, name = name, qq = qq, message = message))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(message = "private")]
pub struct PrivateMessage {
    pub sub_type: String,
    pub message_id: i32,
    pub user_id: i64,
    pub message: MessageChainType,
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
        format!("{}", t!("console.event.message.private", name = name, qq = qq, message = message))
    }
}