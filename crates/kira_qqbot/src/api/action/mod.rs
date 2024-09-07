pub mod ret;

use rust_i18n::t;
use serde::Serialize;
use kira_framework::network::message_chain::MessageChain;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotAction;
use crate::api::anonymous::AnonymousMessage;
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
        format!("{}", t!("console.api.send.private", message = message))
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
        format!("{}", t!("console.api.send.group", group = group_id, message = message))
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
            format!("{}", t!("console.api.send.group", group = group_id, message = message))
        }else {
            format!("{}", t!("console.api.send.private", message = message))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct DeleteMsg {
    pub message_id: i32,
}

impl KiraPrettyDebug for DeleteMsg {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.api.delete", id = self.message_id))
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
        format!("{}", t!("console.api.send_like", qq = self.user_id, times = self.times))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: bool,
}

impl KiraPrettyDebug for SetGroupKick {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.api.set_group_kick", group = self.group_id, qq = self.user_id))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    pub duration: i64,
}

impl KiraPrettyDebug for SetGroupBan {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.api.set_group_ban", group = self.group_id, qq = self.user_id, duration = self.duration))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<AnonymousMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_flag: Option<String>,
    pub duration: i64,
}

impl KiraPrettyDebug for SetGroupAnonymousBan {
    fn pretty_debug(&self) -> String {
        if self.anonymous.is_some() {
            format!("{}", t!("console.api.set_group_anonymous_ban", group = self.group_id, name = self.anonymous.clone().unwrap().name, duration = self.duration))
        }else {
            format!("{}", t!("console.api.set_group_anonymous_ban", group = self.group_id, name = "<Unknown>", duration = self.duration))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupWholeBan {
    pub group_id: i64,
    pub enable: bool,
}

impl KiraPrettyDebug for SetGroupWholeBan {
    fn pretty_debug(&self) -> String {
        if self.enable {
            format!("{}", t!("console.api.set_group_whole_ban.enable", group = self.group_id))
        }else {
            format!("{}", t!("console.api.set_group_whole_ban.disable", group = self.group_id))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupAdmin {
    pub group_id: i64,
    pub user_id: i64,
    pub enable: bool,
}

impl KiraPrettyDebug for SetGroupAdmin {
    fn pretty_debug(&self) -> String {
        if self.enable {
            format!("{}", t!("console.api.set_group_admin.enable", group = self.group_id, qq = self.user_id))
        }else {
            format!("{}", t!("console.api.set_group_admin.disable", group = self.group_id, qq = self.user_id))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupAnonymous {
    pub group_id: i64,
    pub enable: bool,
}

impl KiraPrettyDebug for SetGroupAnonymous {
    fn pretty_debug(&self) -> String {
        if self.enable {
            format!("{}", t!("console.api.set_group_anonymous.enable", group = self.group_id))
        }else {
            format!("{}", t!("console.api.set_group_anonymous.disable", group = self.group_id))
        }
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
        format!("{}", t!("console.api.set_group_card", group = self.group_id, qq = self.user_id, name = self.card))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

impl KiraPrettyDebug for SetGroupName {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.api.set_group_name", group = self.group_id, name = self.group_name))
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupLeave {
    pub group_id: i64,
    pub is_dismiss: bool
}

impl KiraPrettyDebug for SetGroupLeave {
    fn pretty_debug(&self) -> String {
        if self.is_dismiss {
            format!("{}", t!("console.api.set_group_leave.dismiss", group = self.group_id))
        }else {
            format!("{}", t!("console.api.set_group_leave.quit", group = self.group_id))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: String,
    pub duration: i64,
}

impl KiraPrettyDebug for SetGroupSpecialTitle {
    fn pretty_debug(&self) -> String {
        if self.special_title.is_empty() {
            format!("{}", t!("console.api.set_group_special_title.unset", group = self.group_id, qq = self.user_id))
        }else {
            format!("{}", t!("console.api.set_group_special_title", group = self.group_id, qq = self.user_id, title = self.special_title))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: bool,
    pub remark: String,
}

impl KiraPrettyDebug for SetFriendAddRequest {
    fn pretty_debug(&self) -> String {
        if self.approve {
            if self.remark.is_empty() {
                format!("{}", t!("console.api.set_friend_add_request.approve.empty"))
            }else {
                format!("{}", t!("console.api.set_friend_add_request.approve.remark", remark = self.remark))
            }
        }else {
            format!("{}", t!("console.api.set_friend_add_request.reject"))
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: String,
    pub approve: bool,
    pub reason: String,
}

impl KiraPrettyDebug for SetGroupAddRequest {
    fn pretty_debug(&self) -> String {
        if self.approve {
            match self.sub_type.as_str() {
                "add" => format!("{}", t!("console.api.set_group_add_request.add.approve")),
                "invite" => format!("{}", t!("console.api.set_group_add_request.invite.approve")),
                _ => "".to_string()
            }
        }else {
            let reason = if self.reason.is_empty() {
                "<Empty>".to_string()
            }else {
                format!("{}", self.reason)
            };
            match self.sub_type.as_str() {
                "add" => format!("{}", t!("console.api.set_group_add_request.add.reject", reason = reason)),
                "invite" => format!("{}", t!("console.api.set_group_add_request.invite.reject", reason = reason)),
                _ => "".to_string()
            }
        }
    }
}

#[derive(Serialize, OneBotAction)]
pub struct GetLoginInfo;

impl KiraPrettyDebug for GetLoginInfo {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}