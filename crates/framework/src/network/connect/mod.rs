use async_trait::async_trait;
use log::info;
use tokio_tungstenite::tungstenite::Message;
use ur_ecs::resource::Resource;
use ur_ecs::world::World;
use crate::network::actions::{OneBotActionReturn, OneBotActionReturnTrait, OneBotActionTrait};
use crate::network::events::OneBotEventTrait;
use crate::pretty_debug::KiraPrettyDebug;
use std::sync::Arc;
use atomic_mut::AtomicMut;

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
    connect: Arc<AtomicMut<Box<dyn OneBotConnectTrait + Sync + Send>>>,
    world: Option<World>,
}

impl OneBotConnect {
    pub fn new<T: OneBotConnectTrait + Sync + Send + 'static>(connect: T) -> Self {
        Self {
            connect: Arc::new(AtomicMut::new(Box::new(connect))),
            world: None,
        }
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn world(&self) -> World {
        self.world.clone().unwrap()
    }

    pub async fn connect(&self) -> anyhow::Result<()> {
        self.connect.write().connect().await
    }

    pub async fn send_action<E: OneBotActionTrait + Send + Sync + 'static>(&self, action: E) -> anyhow::Result<()> {
        self.connect.read().send_action(Box::new(action)).await
    }

    pub async fn recv(&self) -> anyhow::Result<Message> {
        self.connect.read().recv().await
    }

    pub async fn recv_return<T: OneBotActionReturnTrait + Send + Sync + 'static + KiraPrettyDebug>(&self) -> anyhow::Result<T> {
        let ret = self.connect.read().recv_return().await?.get_data::<T>()?;
        let pretty_debug = KiraPrettyDebug::pretty_debug(&ret);
        if !pretty_debug.is_empty() {
            info!("{}", pretty_debug);
        }
        Ok(ret)
    }
}