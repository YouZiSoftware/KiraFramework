use std::fmt::Debug;
use tokio::task::JoinHandle;
use crate::network::connect::{OneBotConnect};
use crate::network::events::OneBotEventsEnumTrait;
use log::{debug, info};
use ur_ecs::async_manager::{async_system, URECSAsync};
use ur_ecs::component::Component;
use ur_ecs::params::resource::Res;
use ur_ecs::world::World;
use crate::pretty_debug::KiraPrettyDebugToggle;

#[derive(Component)]
pub struct KiraRecvEventLoop {
    pub event_loop: JoinHandle<()>,
}

pub(crate) fn spawn_recv_loop<T: OneBotEventsEnumTrait + Debug + Send + Sync + 'static>(w: World,
                                                                                              connect: Res<OneBotConnect>,
                                                                                              debug_enabled: Res<KiraPrettyDebugToggle>) {
    let world = w.clone();
    let runtime = URECSAsync::runtime();
    let event_loop = runtime.spawn(async move {
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
                        if debug_enabled.0 {
                            let pretty_debug = event.pretty_debug();
                            if !pretty_debug.is_empty() {
                                info!("{}", pretty_debug);
                            }
                        }
                        debug!("Recv event >> {:?}", event);
                        let _ = event.send_event(&world).await;
                    }else {
                        debug!("Recv >> {:?}", message);
                        debug!("Err >> {}", result.err().unwrap());
                    }
                }
            }
        }
        println!("x");
    });
    w.spawn(KiraRecvEventLoop {
        event_loop,
    });
}