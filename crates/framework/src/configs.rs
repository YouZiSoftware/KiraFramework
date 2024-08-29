use bevy_ecs::system::Resource;
use serde::Serialize;

#[derive(Resource, Debug, Clone)]
pub struct BotConfigs {
    pub bot_id: u32,
}

impl BotConfigs {
    pub fn to_self(&self) -> OneBotEventSelf {
        OneBotEventSelf {
            platform: "qq".to_string(),
            user_id: self.bot_id.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct OneBotEventSelf {
    pub platform: String,
    pub user_id: String
}