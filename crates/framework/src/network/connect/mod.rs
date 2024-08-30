use std::sync::Arc;
use async_trait::async_trait;
use bevy_ecs::prelude::Resource;
use log::info;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;
use crate::network::actions::{OneBotActionReturn, OneBotActionReturnTrait, OneBotActionTrait};
use crate::network::events::OneBotEventTrait;
use crate::pretty_debug::KiraPrettyDebug;

pub mod reverse;

#[async_trait]
pub trait OneBotConnectTrait {
    async fn connect(&mut self) -> anyhow::Result<()>;
    async fn send_event(&self, event: Box<dyn OneBotEventTrait + Send + Sync>) -> anyhow::Result<()>;
    async fn send_action(&self, action: Box<dyn OneBotActionTrait + Send + Sync>) -> anyhow::Result<()>;
    async fn recv(&self) -> anyhow::Result<Message>;
    async fn recv_return(&self) -> anyhow::Result<OneBotActionReturn>;
}

#[derive(Resource, Clone)]
pub struct OneBotConnect {
    connect: Arc<RwLock<Box<dyn OneBotConnectTrait + Sync + Send>>>,
}

impl OneBotConnect {
    pub fn new<T: OneBotConnectTrait + Sync + Send + 'static>(connect: T) -> Self {
        Self {
            connect: Arc::new(RwLock::new(Box::new(connect)))
        }
    }

    pub async fn connect(&self) -> anyhow::Result<()> {
        self.connect.write().await.connect().await
    }

    pub async fn send_action<E: OneBotActionTrait + Send + Sync + 'static>(&self, action: E) -> anyhow::Result<()> {
        self.connect.read().await.send_action(Box::new(action)).await
    }

    pub async fn recv(&self) -> anyhow::Result<Message> {
        self.connect.read().await.recv().await
    }

    pub async fn recv_return<T: OneBotActionReturnTrait + Send + Sync + 'static + KiraPrettyDebug>(&self) -> anyhow::Result<T> {
        let ret = self.connect.read().await.recv_return().await?.get_data::<T>()?;
        let pretty_debug = KiraPrettyDebug::pretty_debug(&ret);
        if !pretty_debug.is_empty() {
            info!("{}", pretty_debug);
        }
        Ok(ret)
    }
}