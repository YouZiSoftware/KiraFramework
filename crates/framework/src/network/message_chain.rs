use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait MessageTrait {
    fn get_type() -> String;
    fn get_data(&self) -> Value;
    fn as_persistent_string(&self) -> String;
}

pub trait MessagesEnumTrait {
    fn as_persistent_string(message_type: String, data: Value) -> anyhow::Result<String>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: Value,
    #[serde(skip)]
    pub persistent_string: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageChain(Vec<Message>);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(untagged)]
pub enum MessageChainType {
    #[default]
    None,
    Str(String),
    Chain(MessageChain),
}

impl MessageChain {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(vec: Vec<Message>) -> Self {
        Self(vec)
    }

    pub fn push<T: MessageTrait>(&mut self, message: T) {
        self.0.push(Message {
            message_type: T::get_type(),
            data: message.get_data(),
            persistent_string: message.as_persistent_string(),
        });
    }

    pub fn start_with<T: MessageTrait>(&self) -> bool {
        self.0.get(0).map_or(false, |m| m.message_type == T::get_type())
    }

    pub fn get<T: MessageTrait>(&self, index: usize) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut i = 0;
        for msg in self.0.iter() {
            if msg.message_type == T::get_type() {
                if i == index {
                    return serde_json::from_value::<T>(msg.data.clone()).ok();
                } else {
                    i += 1;
                }
            }
        }
        None
    }

    pub fn remove<T: MessageTrait>(&mut self, index: usize) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut i = 0;
        for msg_i in 0..self.0.len() {
            if self.0[msg_i].message_type == T::get_type() {
                if i == index {
                    let data = self.0.remove(msg_i).data;
                    return serde_json::from_value::<T>(data).ok();
                } else {
                    i += 1;
                }
            }
        }
        None
    }

    pub fn as_persistent_string<T: MessagesEnumTrait>(&self) -> String {
        let mut result = "".to_string();
        for msg in self.0.iter() {
            if let Ok(s) = T::as_persistent_string(msg.message_type.clone(), msg.data.clone()) {
                result += &s;
            }
        }
        result
    }
}