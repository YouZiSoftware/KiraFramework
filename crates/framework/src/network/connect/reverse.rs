use std::str::FromStr;
use std::time::Duration;
use async_trait::async_trait;
use flume::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use log::{debug, info, warn};
use rust_i18n::t;
use tokio::task::JoinHandle;
use tokio::time;
use tokio_tungstenite::tungstenite::{ClientRequestBuilder, Message};
use crate::network::actions::{OneBotAction, OneBotActionReturn, OneBotActionTrait};
use crate::network::connect::OneBotConnectTrait;
use crate::network::events::{OneBotEventTrait};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::Uri;

pub struct OneBotReverseWebSocketConnection {
    pub url: String,
    pub access_token: Option<String>,
    pub reconnect_interval: Duration,
    packet_receiver: Option<Receiver<Message>>,
    return_receiver: Option<Receiver<OneBotActionReturn>>,
    packet_sender: Option<Sender<Message>>,
}

impl OneBotReverseWebSocketConnection {
    pub async fn connect(&mut self) -> anyhow::Result<(JoinHandle<()>, JoinHandle<()>)> {
        let (receiver_sender, packet_receiver) = flume::unbounded();
        let (return_sender, return_receiver) = flume::unbounded();
        let (packet_sender, sender_receiver) = flume::unbounded();

        let mut uri = ClientRequestBuilder::new(Uri::from_str(&format!("ws://{}", self.url))?);
        if let Some(access_token) = &self.access_token {
            uri = uri.with_header("Authorization", format!("Bearer {}", access_token));
        }

        let req = uri.into_client_request()?;
        let (ws_stream, _) = tokio_tungstenite::connect_async(req).await?;
        let (mut write, mut read) = ws_stream.split();

        let recv_loop = tokio::spawn(async move {
            while let Some(Ok(packet)) = read.next().await {
                let p = packet.clone();
                if let Ok(message) = p.into_text() {
                    if let Ok(ret) = serde_json::from_str::<OneBotActionReturn>(&message) {
                        debug!("recv return >> {}", message);
                        let _ = return_sender.send_async(ret).await;
                        continue
                    }
                }
                let _ = receiver_sender.send_async(packet).await;
            }
        });
        let send_loop = tokio::spawn(async move {
            while let Ok(packet) = sender_receiver.recv_async().await {
                let _ = write.send(packet).await;
            }
        });

        self.packet_receiver = Some(packet_receiver);
        self.packet_sender = Some(packet_sender);
        self.return_receiver = Some(return_receiver);

        Ok((recv_loop, send_loop))
    }

    pub async fn send_action(&self, action: Box<dyn OneBotActionTrait + Send + Sync>) -> anyhow::Result<()> {
        let json = serde_json::to_string(&OneBotAction {
            action: action.get_action(),
            params: action.get_data(),
        })?;
        debug!("send action >> {}", &json);
        let pretty_debug = action.pretty_debug();
        if !pretty_debug.is_empty() {
            info!("{}", pretty_debug);
        }
        self.packet_sender
            .as_ref()
            .ok_or(anyhow::anyhow!("packet_sender is None")).unwrap()
            .send_async(Message::Text(json)).await?;
        Ok(())
    }

    pub async fn recv(&self) -> anyhow::Result<Message> {
        self.packet_receiver
            .as_ref()
            .ok_or(anyhow::anyhow!("packet_receiver is None"))?
            .recv_async()
            .await
            .map_err(|error| anyhow::anyhow!("recv error: {}", error))
    }

    pub async fn recv_return(&self) -> anyhow::Result<OneBotActionReturn> {
        self.return_receiver
            .as_ref()
            .ok_or(anyhow::anyhow!("return_receiver is None"))?
            .recv_async()
            .await
            .map_err(|error| anyhow::anyhow!("recv return error: {}", error))
    }
}

pub struct OneBotReverseWebSocket {
    connection: OneBotReverseWebSocketConnection,
    recv_loop: Option<JoinHandle<()>>,
    send_loop: Option<JoinHandle<()>>,
}

impl OneBotReverseWebSocket {
pub fn new(url: &str, access_token: Option<&str>, reconnect_interval: Duration) -> Self {
    Self {
        connection: OneBotReverseWebSocketConnection {
            url: url.to_string(),
            access_token: access_token.map(|s| s.to_string()),
            reconnect_interval,
            packet_receiver: None,
            return_receiver: None,
            packet_sender: None,
        },
        recv_loop: None,
        send_loop: None,
    }
}
}

#[async_trait]
impl OneBotConnectTrait for OneBotReverseWebSocket {
    async fn connect(&mut self) -> anyhow::Result<()> {
        loop {
            if let Ok((recv_loop, send_loop)) = self.connection.connect().await {
                self.recv_loop = Some(recv_loop);
                self.send_loop = Some(send_loop);
                info!("{}", t!("console.connected"));
                break
            } else {
                let sec = self.connection.reconnect_interval.as_secs();
                warn!("{}", t!("console.retry", sec = sec));
                time::sleep(self.connection.reconnect_interval).await;
                continue
            }
        }
        Ok(())
    }

    async fn send_event(&self, _event: Box<dyn OneBotEventTrait + Send + Sync>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn send_action(&self, action: Box<dyn OneBotActionTrait + Send + Sync>) -> anyhow::Result<()> {
        self.connection.send_action(action).await
    }
    async fn recv(&self) -> anyhow::Result<Message> {
        self.connection.recv().await
    }

    async fn recv_return(&self) -> anyhow::Result<OneBotActionReturn> {
        self.connection.recv_return().await
    }
}