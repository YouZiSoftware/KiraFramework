use rust_i18n::t;
use serde::Deserialize;
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::OneBotActionReturn;

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