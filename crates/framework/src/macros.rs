#[macro_export]
macro_rules! kira_async {
    ($expr: expr => $block: block) => {
        let runtime = KiraAsyncManager::global().runtime($expr).unwrap();
        runtime.spawn(async move {
            $block
        });
    };
    ($expr: expr => $code: expr) => {
        let runtime = KiraAsyncManager::global().runtime($expr).unwrap();
        runtime.spawn(async move {
            $code
        });
    };
}

#[macro_export]
macro_rules! kira_recv {
    ($recv: ident(let $var: ident) => $block: block) => {
        for $var in $recv.read() {
            let $var = $var.event.clone();
            $block
        }
    };
}

#[macro_export]
macro_rules! kira_timeout {
    ($timeout: expr => $block: block) => {
        tokio::time::timeout($timeout, async move {
            $block
        }).await
    }
}

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