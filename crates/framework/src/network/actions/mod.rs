use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct OneBotAction {
    pub action: String,
    pub params: Value,
}

pub trait OneBotActionTrait {
    fn get_action(&self) -> String;
    fn get_data(&self) -> Value;
    fn pretty_debug(&self) -> String;
}

#[derive(Deserialize, Debug)]
pub struct OneBotActionReturn {
    pub status: String,
    pub retcode: i32,
    pub data: Value,
    pub echo: Option<Value>,
}

impl OneBotActionReturn {
    pub fn get_data<T: OneBotActionReturnTrait>(&self) -> Result<T, OneBotActionReturnError> {
        if self.status == "ok" && self.retcode == 0 {
            T::from_json(self.data.clone()).map_err(|_| OneBotActionReturnError {
                status: self.status.clone(),
                retcode: self.retcode,
            })
        } else {
            Err(OneBotActionReturnError {
                status: self.status.clone(),
                retcode: self.retcode,
            })
        }
    }
}

pub trait OneBotActionReturnTrait {
    fn from_json(json: Value) -> anyhow::Result<Self> where Self: Sized;
}

#[derive(Debug)]
pub struct OneBotActionReturnError {
    pub status: String,
    pub retcode: i32,
}

impl Display for OneBotActionReturnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
impl Error for OneBotActionReturnError {}