use rust_i18n::t;
use serde::{Deserialize, Serialize};
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotEvent;
use crate::api::file::QQFile;
use kira_framework_proc::onebot_event_type;

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_upload")]
pub struct GroupUpload {
    pub group_id: i64,
    pub user_id: i64,
    pub file: QQFile,
}

impl KiraPrettyDebug for GroupUpload {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_upload", group = self.group_id, qq = self.user_id, file_name = self.file.name, file_size = self.file.size))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_admin.set")]
pub struct GroupAdminSet {
    pub group_id: i64,
    pub user_id: i64,
}

impl KiraPrettyDebug for GroupAdminSet {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_admin.set", group = self.group_id, qq = self.user_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_admin.unset")]
pub struct GroupAdminUnset {
    pub group_id: i64,
    pub user_id: i64,
}

impl KiraPrettyDebug for GroupAdminUnset {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_admin.unset", group = self.group_id, qq = self.user_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_decrease.leave")]
pub struct GroupDecreaseLeave {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupDecreaseLeave {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_decrease.leave", group = self.group_id, qq = self.user_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_decrease.kick")]
pub struct GroupDecreaseKick {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupDecreaseKick {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_decrease.kick", group = self.group_id, qq = self.user_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_decrease.kick_me")]
pub struct GroupDecreaseKickMe {
    pub group_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupDecreaseKickMe {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_decrease.kick_me", group = self.group_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_increase.approve")]
pub struct GroupIncreaseApprove {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupIncreaseApprove {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_increase.approve", group = self.group_id, qq = self.user_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_increase.invite")]
pub struct GroupIncreaseInvite {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupIncreaseInvite {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_increase.invite", group = self.group_id, qq = self.user_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_ban.ban")]
pub struct GroupBan {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
    pub duration: i64,
}

impl KiraPrettyDebug for GroupBan {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_ban.ban", group = self.group_id, qq = self.user_id, op_qq = self.operator_id, duration = self.duration))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_ban.lift_ban")]
pub struct GroupLiftBan {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
}

impl KiraPrettyDebug for GroupLiftBan {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_ban.lift_ban", group = self.group_id, qq = self.user_id, op_qq = self.operator_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "friend_add")]
pub struct FriendAdd {
    pub user_id: i64,
}

impl KiraPrettyDebug for FriendAdd {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "group_recall")]
pub struct GroupRecall {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
    pub message_id: i64,
}

impl KiraPrettyDebug for GroupRecall {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.group_recall", group = self.group_id, qq = self.user_id, op_qq = self.operator_id, id = self.message_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "friend_recall")]
pub struct FriendRecall {
    pub user_id: i64,
    pub message_id: i64,
}

impl KiraPrettyDebug for FriendRecall {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.friend_recall", qq = self.user_id, id = self.message_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "notify.poke")]
pub struct NotifyPoke {
    pub group_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

impl KiraPrettyDebug for NotifyPoke {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.notify.poke", group = self.group_id, qq = self.target_id, op_qq = self.user_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "notify.lucky_king")]
pub struct NotifyLuckyKing {
    pub group_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

impl KiraPrettyDebug for NotifyLuckyKing {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.notice.notify.lucky_king", group = self.group_id, qq = self.user_id, op_qq = self.target_id))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(notice = "notify.honor")]
pub struct NotifyHonor {
    pub group_id: i64,
    pub honor_type: String,
    pub user_id: i64,
}

impl KiraPrettyDebug for NotifyHonor {
    fn pretty_debug(&self) -> String {
        match self.honor_type.as_str() {
            "talkative" => format!("{}", t!("console.event.notice.notify.honor.talkative", group = self.group_id, qq = self.user_id)),
            "performer" => format!("{}", t!("console.event.notice.notify.honor.performer", group = self.group_id, qq = self.user_id)),
            "emotion" => format!("{}", t!("console.event.notice.notify.honor.emotion", group = self.group_id, qq = self.user_id)),
            _ => "".to_string(),
        }
    }
}