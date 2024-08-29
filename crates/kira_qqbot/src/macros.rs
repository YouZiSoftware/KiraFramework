use std::path::PathBuf;
use crate::messages::{Image, Reply};
#[macro_export]
macro_rules! text {
    () => {
        Text::new("")
    };
    ($($arg:tt)*) => {{
        Text {
            text: format!($($arg)*)
        }
    }};
}

#[macro_export]
macro_rules! at {
    ($qq:expr) => {{
        let str = {
            $qq
        };
        let str = str.to_string();
        At::from_str(str)
    }};
}

#[macro_export]
macro_rules! reply {
    ($id:expr) => {{
        let str = {
            $id
        };
        let str = str.to_string();
        Reply {
            id: str
        }
    }};
}

#[macro_export]
macro_rules! image {
    (file($path: expr)) => {{
        let path = PathBuf::from($path);
        Image::file(path)
    }};
}