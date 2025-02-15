pub use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use ur_ecs::app::App;
use ur_ecs::event::{Event, EventId};
use ur_ecs::world::World;
pub use kira_framework_proc::{ OneBotEvent, OneBotEventsEnum };

#[derive(Event, Debug, Clone)]
pub struct OneBotEventReceiver<T: OneBotEventTrait + Send + Sync + Sized + Clone> {
    pub event: T,
}

impl<T: OneBotEventTrait + Send + Sync + Sized + Clone> OneBotEventReceiver<T> {
    pub fn new(event: T) -> Self {
        Self { event }
    }
}

#[derive(Serialize, Default)]
pub struct OneBotEvent {
    pub self_id: i64,
    pub time: i64,
    pub post_type: String,
}

impl OneBotEvent {
    pub fn build(self_id: i64, post_type: String) -> Value {
        //获取16位时间戳, 并转为10位时间戳
        let time = chrono::Local::now().timestamp_millis();
        serde_json::to_value(OneBotEvent {
            self_id,
            time,
            post_type,
        }).unwrap()
    }
}

#[async_trait]
pub trait OneBotEventTrait where Self: 'static {
    async fn send_event(self, world: &World) -> anyhow::Result<EventId>
    where Self: Send + Sync + Sized + Clone;
    fn to_json(&self) -> anyhow::Result<Value>;
}

#[async_trait]
pub trait OneBotEventsEnumTrait {
    async fn send_event(self, world: &World) -> anyhow::Result<()>;
    fn from_json(json: String) -> anyhow::Result<Self> where Self: Sized;
    fn add_events(app: &App);
    fn pretty_debug(&self) -> String;
}

pub trait OneBotEventTypeTrait {
    fn get_post_type() -> String;
    fn get_sub_type() -> String;
    fn get_type_value() -> String;
}