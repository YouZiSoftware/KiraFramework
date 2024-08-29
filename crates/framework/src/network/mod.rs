use std::fmt::Debug;
use bevy_ecs::system::Resource;
use serde::Serialize;
pub mod connect;
pub mod events;
pub mod actions;
pub mod message_chain;

#[derive(Serialize, Debug, Clone)]
pub struct OneBotVersion {
    #[serde(rename = "impl")]
    pub version_impl: String,
    pub version: String,
    pub onebot_version: String,
}

impl Default for OneBotVersion {
    fn default() -> Self {
        Self {
            version_impl: "kira-qqbot-framework".to_string(),
            version: "0.2.0".to_string(),
            onebot_version: "11".to_string(),
        }
    }
}