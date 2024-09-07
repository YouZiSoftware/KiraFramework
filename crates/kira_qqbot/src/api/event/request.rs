use rust_i18n::t;
use serde::{Deserialize, Serialize};
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotEvent;
use kira_framework_proc::onebot_event_type;

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(request = "friend")]
pub struct FriendRequest {
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

impl KiraPrettyDebug for FriendRequest {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.request.friend", qq = self.user_id, comment = self.comment))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(request = "group.add")]
pub struct GroupAddRequest {
    pub group_id: i64,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

impl KiraPrettyDebug for GroupAddRequest {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.request.group.add", group = self.group_id, qq = self.user_id, comment = self.comment))
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(request = "group.invite")]
pub struct GroupInviteRequest {
    pub group_id: i64,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

impl KiraPrettyDebug for GroupInviteRequest {
    fn pretty_debug(&self) -> String {
        format!("{}", t!("console.event.request.group.invite", group = self.group_id, qq = self.user_id, comment = self.comment))
    }
}