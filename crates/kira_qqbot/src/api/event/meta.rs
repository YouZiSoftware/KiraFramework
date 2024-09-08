use serde::{Deserialize, Serialize};
use kira_framework::pretty_debug::KiraPrettyDebug;
use kira_framework_proc::{onebot_event_type, OneBotEvent};

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.enable")]
pub struct LifecycleEnable {
    pub time: i64,
}

impl KiraPrettyDebug for LifecycleEnable {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.disable")]
pub struct LifecycleDisable {
    pub time: i64,
}

impl KiraPrettyDebug for LifecycleDisable {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.connect")]
pub struct LifecycleConnect {
    pub time: i64,
}

impl KiraPrettyDebug for LifecycleConnect {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "heartbeat")]
pub struct Heartbeat {
    interval: i64,
}

impl KiraPrettyDebug for Heartbeat {
    fn pretty_debug(&self) -> String {
        "".to_string()
    }
}