#[macro_export]
macro_rules! text {
    () => {
        $crate::messages::Text::new("")
    };
    ($($arg:tt)*) => {{
        $crate::messages::Text {
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
        $crate::messages::At::from_str(str)
    }};
}

#[macro_export]
macro_rules! reply {
    ($id:expr) => {{
        let str = {
            $id
        };
        let str = str.to_string();
        $crate::messages::Reply {
            id: str
        }
    }};
}

#[macro_export]
macro_rules! image {
    (file($path: expr)) => {{
        let path = PathBuf::from($path);
        $crate::messages::Image::file(path)
    }};
}

#[macro_export]
macro_rules! xml {
    ($id:expr) => {{
        let str = {
            $id
        };
        let data = str.to_string();
        $crate::messages::Xml {
            data
        }
    }};
}

#[macro_export]
macro_rules! json {
    ($id:expr) => {{
        let str = {
            $id
        };
        let data = str.to_string();
        $crate::messages::Json {
            data
        }
    }};
}