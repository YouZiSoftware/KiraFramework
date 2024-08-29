use bevy_app::App;
use bevy_ecs::event::{Event, EventId};
use bevy_ecs::world::World;
use serde::Serialize;
use serde_json::Value;

#[derive(Event, Debug)]
pub struct OneBotEventReceiver<T: OneBotEventTrait + Send + Sync + Sized> {
    pub event: T,
}

impl<T: OneBotEventTrait + Send + Sync + Sized> OneBotEventReceiver<T> {
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

pub trait OneBotEventTrait where Self: 'static {
    fn send_event(self, world: &mut World) -> anyhow::Result<EventId<OneBotEventReceiver<Self>>>
    where Self: Send + Sync + Sized;
    fn to_json(&self) -> anyhow::Result<Value>;
}

pub trait OneBotEventsEnumTrait {
    fn send_event(self, world: &mut World) -> anyhow::Result<()>;
    fn from_json(json: String) -> anyhow::Result<Self> where Self: Sized;
    fn add_events(app: &mut App);
    fn pretty_debug(&self) -> String;
}

pub trait OneBotEventTypeTrait {
    fn get_post_type() -> String;
    fn get_type_value() -> String;
}