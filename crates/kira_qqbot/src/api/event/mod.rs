use kira_framework_proc::OneBotEventsEnum;
use log::debug;
use crate::api::event::message::*;
use crate::api::event::meta::*;
use crate::api::event::notice::*;
use crate::api::event::request::*;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(OneBotEventsEnum, Debug)]
pub enum OneBotEvents {
    //Message
    GroupMessage(GroupMessage),
    PrivateMessage(PrivateMessage),
    //Meta
    LifecycleEnable(LifecycleEnable),
    LifecycleDisable(LifecycleDisable),
    LifecycleConnect(LifecycleConnect),
    Heartbeat(Heartbeat),
    //Notice
    GroupUpload(GroupUpload),
    GroupAdminSet(GroupAdminSet),
    GroupAdminUnset(GroupAdminUnset),
    GroupDecreaseLeave(GroupDecreaseLeave),
    GroupDecreaseKick(GroupDecreaseKick),
    GroupDecreaseKickMe(GroupDecreaseKickMe),
    GroupIncreaseApprove(GroupIncreaseApprove),
    GroupIncreaseInvite(GroupIncreaseInvite),
    GroupBan(GroupBan),
    GroupLiftBan(GroupLiftBan),
    FriendAdd(FriendAdd),
    GroupRecall(GroupRecall),
    FriendRecall(FriendRecall),
    NotifyPoke(NotifyPoke),
    NotifyLuckyKing(NotifyLuckyKing),
    NotifyHonor(NotifyHonor),
    //Request
    FriendRequest(FriendRequest),
    GroupAddRequest(GroupAddRequest),
    GroupInviteRequest(GroupInviteRequest),
}