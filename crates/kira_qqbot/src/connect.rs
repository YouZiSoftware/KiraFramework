use std::time::Duration;
use kira_framework::network::connect::OneBotConnect;
use kira_framework::network::events::{OneBotEventReceiver, OneBotEventTrait};
use kira_framework::network::message_chain::MessageChain;
use crate::api::action::ret::{EmptyReturn, GetForwardMsgReturn, GetLoginInfoReturn, GetMsgReturn, SendMsgReturn};
use crate::api::action::{DeleteMsg, GetForwardMsg, GetLoginInfo, GetMsg, SendGroupMsg, SendLike, SetFriendAddRequest, SetGroupAddRequest, SetGroupAdmin, SetGroupAnonymous, SetGroupAnonymousBan, SetGroupBan, SetGroupCard, SetGroupKick, SetGroupLeave, SetGroupName, SetGroupSpecialTitle, SetGroupWholeBan};
use crate::api::anonymous::AnonymousMessage;

#[derive(Clone)]
pub struct KiraQQBotConnect {
    connect: OneBotConnect
}

impl KiraQQBotConnect {
    pub fn new(connect: OneBotConnect) -> Self {
        KiraQQBotConnect {
            connect
        }
    }

    pub async fn wait_event<T: OneBotEventTrait + Send + Sync + Sized + Clone>(&self) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let world = self.connect.world();
        let event: OneBotEventReceiver<T> = world.wait_for_event().await;
        Ok(event.event)
    }

    pub async fn send_group_message(&self, group_id: i64, message: MessageChain, auto_escape: bool) -> anyhow::Result<i32> {
        self.connect.send_action(SendGroupMsg {
            group_id,
            message,
            auto_escape
        }).await?;
        let message_id = self.connect.recv_return::<SendMsgReturn>().await?.message_id;
        message_id.ok_or(anyhow::anyhow!("message_id is None"))
    }

    pub async fn recall_message(&self, message_id: i32) -> anyhow::Result<()> {
        self.connect.send_action(DeleteMsg {
            message_id
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn get_message(&self, message_id: i32) -> anyhow::Result<GetMsgReturn> {
        self.connect.send_action(GetMsg {
            message_id
        }).await?;
        self.connect.recv_return::<GetMsgReturn>().await
    }

    pub async fn get_forward_message(&self, id: String) -> anyhow::Result<MessageChain> {
        self.connect.send_action(GetForwardMsg {
            id
        }).await?;
        self.connect.recv_return::<GetForwardMsgReturn>().await.map(|x| x.message)
    }

    pub async fn send_like(&self, user_id: i64, times: i8) -> anyhow::Result<()> {
        self.connect.send_action(SendLike {
            user_id,
            times,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_kick(&self, group_id: i64, user_id: i64, reject_add_request: bool) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupKick {
            group_id,
            user_id,
            reject_add_request,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_ban(&self, group_id: i64, user_id: i64, duration: Option<Duration>) -> anyhow::Result<()> {
        let duration = if let Some(duration) = duration {
            duration.as_secs() as i64
        }else {
            0
        };
        self.connect.send_action(SetGroupBan {
            group_id,
            user_id,
            duration,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_anonymous_ban(&self, group_id: i64, anonymous: AnonymousMessage, duration: Option<Duration>) -> anyhow::Result<()> {
        let duration = if let Some(duration) = duration {
            duration.as_secs() as i64
        }else {
            0
        };
        self.connect.send_action(SetGroupAnonymousBan {
            group_id,
            anonymous: Some(anonymous),
            anonymous_flag: None,
            duration,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_whole_ban(&self, group_id: i64, enable: bool) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupWholeBan {
            group_id,
            enable,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_admin(&self, group_id: i64, user_id: i64, enable: bool) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupAdmin {
            group_id,
            user_id,
            enable,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_anonymous(&self, group_id: i64, enable: bool) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupAnonymous {
            group_id,
            enable,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_card(&self, group_id: i64, user_id: i64, card: Option<String>) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupCard {
            group_id,
            user_id,
            card: card.unwrap_or("".to_string()),
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_name(&self, group_id: i64, group_name: String) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupName {
            group_id,
            group_name,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_leave(&self, group_id: i64, is_dismiss: bool) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupLeave {
            group_id,
            is_dismiss,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_special_title(&self, group_id: i64, user_id: i64, special_title: String) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupSpecialTitle {
            group_id,
            user_id,
            special_title,
            duration: -1,
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_friend_add_request(&self, flag: String, approve: bool, remark: Option<String>) -> anyhow::Result<()> {
        self.connect.send_action(SetFriendAddRequest {
            flag,
            approve,
            remark: remark.unwrap_or("".to_string()),
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn set_group_add_request(&self, flag: String, sub_type: String, approve: bool, reason: Option<String>) -> anyhow::Result<()> {
        self.connect.send_action(SetGroupAddRequest {
            flag,
            sub_type,
            approve,
            reason: reason.unwrap_or("".to_string()),
        }).await?;
        let _ = self.connect.recv_return::<EmptyReturn>().await;
        Ok(())
    }

    pub async fn get_login_info(&self) -> anyhow::Result<GetLoginInfoReturn> {
        self.connect.send_action(GetLoginInfo {}).await?;
        self.connect.recv_return::<GetLoginInfoReturn>().await
    }
}