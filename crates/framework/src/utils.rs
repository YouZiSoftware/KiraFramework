use serde_json::{Map, Value};
use crate::network::events::{OneBotEvent, OneBotEventTrait, OneBotEventTypeTrait};

pub struct Utils;

impl Utils {
    pub fn event_to_json<T: OneBotEventTrait + OneBotEventTypeTrait>(event: T, self_id: i64) -> anyhow::Result<Value> {
        let json = event.to_json()?;
        let post_type = T::get_post_type();
        let mut one_event = serde_json::to_value(OneBotEvent::build(self_id, T::get_post_type()))?;
        let one_event_object = one_event.as_object_mut().ok_or(anyhow::anyhow!("json error"))?;
        let json = json.as_object().ok_or(anyhow::anyhow!("json error"))?;
        Self::json_merge(one_event_object, json);
        one_event_object.insert(format!("{}_type", &post_type), Value::String(T::get_type_value()));
        Ok(one_event)
    }
    pub fn json_merge(first_context: &mut Map<String, Value>, second_context: &Map<String, Value>) {
        let mut new_context = Map::new();
        for (key, value) in first_context.iter() {
            new_context.insert(key.clone(), value.clone());
        }
        for (key, value) in second_context.iter() {
            new_context.insert(key.clone(), value.clone());
        }
        *first_context = new_context
    }
}