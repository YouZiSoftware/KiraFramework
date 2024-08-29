use kira_framework_proc::OneBotEventsEnum;
use crate::api::event::message::{GroupMessage, PrivateMessage};

pub mod message;

#[derive(OneBotEventsEnum, Debug)]
pub enum OneBotEvents {
    GroupMessage(GroupMessage),
    PrivateMessage(PrivateMessage),
}