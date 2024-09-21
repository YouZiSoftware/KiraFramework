use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use kira_framework::persistent::AsPersistentStringTrait;
use kira_framework_proc::{AsPersistentString, OneBotMessage, OneBotMessagesEnum};

#[derive(OneBotMessagesEnum)]
pub enum Messages {
    Text,
    At,
    Reply,
    Image,
    Xml,
    Json,
}

#[derive(Serialize, Deserialize, OneBotMessage)]
pub struct Text {
    pub text: String,
}

impl AsPersistentStringTrait for Text {
    fn as_persistent_string(&self) -> String {
        self.text.clone()
    }
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

pub enum AtType {
    QQ(i64),
    All,
}

#[derive(Serialize, Deserialize, OneBotMessage, AsPersistentString)]
pub struct At {
    pub qq: String,
}

impl At {
    pub fn new(at_type: AtType) -> Self {
        match at_type {
            AtType::QQ(qq) => Self { qq: qq.to_string() },
            AtType::All => Self { qq: "all".to_string() },
        }
    }

    pub fn from_str(qq: String) -> Self {
        Self {
            qq,
        }
    }
}

#[derive(Serialize, Deserialize, OneBotMessage, AsPersistentString)]
pub struct Reply {
    pub id: String
}

#[derive(Serialize, Deserialize, OneBotMessage)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

impl Image {
    pub fn file(file: PathBuf) -> Self {
        Self {
            file: Some(format!("file:///{}", file.to_str().unwrap())),
        }
    }
}

impl AsPersistentStringTrait for Image {
    fn as_persistent_string(&self) -> String {
        format!("[CQ:image,file={}]", self.file.as_ref().unwrap())
    }
}

#[derive(Serialize, Deserialize, OneBotMessage, AsPersistentString)]
pub struct Xml {
    pub data: String
}

#[derive(Serialize, Deserialize, OneBotMessage, AsPersistentString)]
pub struct Json {
    pub data: String
}