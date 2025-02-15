#[macro_export]
macro_rules! messages {
    ($($message: expr),+) => {{
        let mut message_chain = MessageChain::new();
        $(
        message_chain.push($message);
        )+
        message_chain
    }};
}