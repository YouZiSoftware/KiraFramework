use kira_framework_proc::OneBotEventsEnum;
use crate::api::event::message::{GroupMessage, PrivateMessage};

pub mod message;
mod meta;
mod notice;
mod request;

#[derive(OneBotEventsEnum, Debug)]
pub enum OneBotEvents {
    GroupMessage(GroupMessage),
    PrivateMessage(PrivateMessage),
}