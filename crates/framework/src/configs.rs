use serde::Serialize;
use ur_ecs::resource::Resource;

#[derive(Resource, Debug, Clone)]
pub struct BotConfigs {
    pub bot_id: i64,
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