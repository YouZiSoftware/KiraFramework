use rust_i18n::t;
use serde::Deserialize;
use kira_framework::network::message_chain::MessageChain;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotActionReturn;
use crate::api::sender::Sender;

#[derive(Deserialize, OneBotActionReturn)]
pub struct EmptyReturn {}

impl KiraPrettyDebug for EmptyReturn {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Deserialize, OneBotActionReturn)]
pub struct SendMsgReturn {
    pub message_id: Option<i32>,
}

impl KiraPrettyDebug for SendMsgReturn {
    fn pretty_debug(&self) -> String {
        if let Some(message_id) = self.message_id {
            format!("{}", t!("console.return.msg", id = message_id))
        }else {
            format!("{}", t!("console.return.msg", id = "Unknown"))
        }
    }
}

#[derive(Deserialize, OneBotActionReturn)]
pub struct GetMsgReturn {
    pub time: i32,
    pub message_type: String,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: Sender,
    pub message: MessageChain,
}

impl KiraPrettyDebug for GetMsgReturn {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Deserialize, OneBotActionReturn)]
pub struct GetForwardMsgReturn {
    pub message: MessageChain,
}

impl KiraPrettyDebug for GetForwardMsgReturn {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}


#[derive(Deserialize, OneBotActionReturn)]
pub struct GetLoginInfoReturn {
    pub user_id: i64,
    pub nickname: String,
}

impl KiraPrettyDebug for GetLoginInfoReturn {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}