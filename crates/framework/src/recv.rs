use std::fmt::Debug;
use bevy_app::{App, Plugin, PreStartup, Startup};
use bevy_ecs::prelude::{Component, IntoSystemConfigs, Mut};
use bevy_ecs::system::Res;
use bevy_ecs::world::{FromWorld, World};
use tokio::task::JoinHandle;
use bevy_async_ecs::{AsyncEcsPlugin, AsyncWorld};
use crate::network::connect::{OneBotConnect, OneBotConnectTrait};
use crate::network::connect::reverse::OneBotReverseWebSocket;
use crate::network::events::OneBotEventsEnumTrait;
use log::{debug, info, LevelFilter, trace};
use crate::startup;
use crate::async_manager::{KiraAsyncManager, KiraAsyncManagerPlugin};
use crate::kira_async;
use crate::pretty_debug::KiraPrettyDebug;

#[derive(Component)]
pub struct KiraRecvEventLoop {
    pub event_loop: JoinHandle<()>,
}

pub(crate) async fn spawn_recv_loop<T: OneBotEventsEnumTrait + Debug + Send + Sync + 'static>(world: AsyncWorld) {
    let connect = world.wait_for_resource::<OneBotConnect>().await;
    connect.connect().await.unwrap();
    loop {
        let message = connect.recv().await;
        if let Ok(message) = message {
            if let Ok(message) = message.into_text() {
                if message.is_empty() {
                    continue
                }
                let result = T::from_json(message.clone());
                if let Ok(event) = result {
                    info!("{}", event.pretty_debug());
                    debug!("recv event >> {:?}", event);
                    world.apply(|world: &mut World| {
                        let _ = event.send_event(world);
                    }).await;
                }else {
                    debug!("recv >> {:?}", message);
                    debug!("err >> {}", result.err().unwrap());
                }
            }
        }
    }
}