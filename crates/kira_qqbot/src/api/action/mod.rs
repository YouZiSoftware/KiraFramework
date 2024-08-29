pub mod ret;

use rust_i18n::t;
use serde::Serialize;
use kira_framework::network::message_chain::MessageChain;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotAction;
use crate::messages::Messages;

#[derive(Serialize, OneBotAction)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: MessageChain,
    pub auto_escape: bool,
}

impl KiraPrettyDebug for SendPrivateMsg {
    fn pretty_debug(&self) -> String {
        let message = self.message.as_persistent_string::<Messages>();
        format!("{}", t!("console.send.private", message = message))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: MessageChain,
    pub auto_escape: bool,
}

impl KiraPrettyDebug for SendGroupMsg {
    fn pretty_debug(&self) -> String {
        let group_id = self.group_id;
        let message = self.message.as_persistent_string::<Messages>();
        format!("{}", t!("console.send.group", group = group_id, message = message))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SendMsg {
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    pub message: MessageChain,
    pub auto_escape: bool,
}

impl KiraPrettyDebug for SendMsg {
    fn pretty_debug(&self) -> String {
        let message = self.message.as_persistent_string::<Messages>();
        if self.message_type == "group" {
            let group_id = self.group_id.unwrap();
            format!("{}", t!("console.send.group", group = group_id, message = message))
        }else {
            format!("{}", t!("console.send.private", message = message))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct DeleteMsg {
    pub message_id: i32,
}

impl KiraPrettyDebug for DeleteMsg {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.delete", id = self.message_id))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct GetMsg {
    pub message_id: i32,
}

impl KiraPrettyDebug for GetMsg {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, OneBotAction)]
pub struct GetForwardMsg {
    pub id: String,
}

impl KiraPrettyDebug for GetForwardMsg {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SendLike {
    pub user_id: i64,
    pub times: i8,
}

impl KiraPrettyDebug for SendLike {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.send_like", qq = self.user_id, times = self.times))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupCard {
    pub group_id: i64,
    pub user_id: i64,
    pub card: String,
}

impl KiraPrettyDebug for SetGroupCard {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.set_group_card", group = self.group_id, qq = self.user_id, name = self.card))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

impl KiraPrettyDebug for SetGroupName {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.set_group_name", group = self.group_id, name = self.group_name))
    }
}