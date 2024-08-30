use kira_framework::network::connect::OneBotConnect;
use kira_framework::network::message_chain::MessageChain;
use crate::api::action::ret::{EmptyReturn, SendMsgReturn};
use crate::api::action::{DeleteMsg, SendGroupMsg};

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
}