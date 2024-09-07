use std::fmt::Debug;
use bevy_ecs::prelude::Component;
use bevy_ecs::world::World;
use tokio::task::JoinHandle;
use bevy_async_ecs::AsyncWorld;
use crate::network::connect::{OneBotConnect};
use crate::network::events::OneBotEventsEnumTrait;
use log::{debug, info};
use crate::pretty_debug::KiraPrettyDebugToggle;

#[derive(Component)]
pub struct KiraRecvEventLoop {
    pub event_loop: JoinHandle<()>,
}

pub(crate) async fn spawn_recv_loop<T: OneBotEventsEnumTrait + Debug + Send + Sync + 'static>(world: AsyncWorld) {
    let connect = world.wait_for_resource::<OneBotConnect>().await;
    let debug_enabled = world.wait_for_resource::<KiraPrettyDebugToggle>().await.0;
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
                    if debug_enabled {
                        info!("{}", event.pretty_debug());
                    }
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