pub mod configs;
pub mod network;
pub mod utils;
pub mod recv;
pub mod async_manager;
pub mod macros;
pub mod pretty_debug;
pub mod persistent;
pub mod lib_plugin;

#[cfg(feature = "derive")]
extern crate kira_framework_proc;

use std::fmt::Debug;
use std::path::Path;
use bevy_app::{App, AppExit, Plugins, ScheduleRunnerPlugin, Startup};
use bevy_async_ecs::{AsyncEcsPlugin, AsyncWorld};
use bevy_ecs::event::Event;
use bevy_ecs::prelude::{IntoSystemConfigs, World};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::Resource;
use bevy_ecs::world::FromWorld;
use rust_i18n::i18n;
use crate::async_manager::{KiraAsyncManager, KiraAsyncManagerPlugin};
use crate::configs::BotConfigs;
use crate::lib_plugin::LibPlugin;
use crate::network::connect::OneBotConnect;
use crate::network::events::OneBotEventsEnumTrait;
use crate::recv::spawn_recv_loop;

i18n!("locales");

pub struct BotApp {
    app: App
}

impl BotApp {
    pub fn new() -> Self {
        BotApp {
            app: App::new(),
        }
    }

    pub fn bot_configs(&mut self, configs: BotConfigs) -> &mut Self {
        self.app.insert_resource(configs);
        self
    }

    pub fn onebot_connect(&mut self, connect: OneBotConnect) -> &mut Self {
        self.app.insert_resource(connect);
        self
    }

    pub fn add_plugins<T>(&mut self, plugin: impl Plugins<T>) -> &mut Self {
        self.app.add_plugins(plugin);
        self
    }

    pub fn add_lib_plugins<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<&mut Self> {
        let path = path.as_ref();
        if path.is_file() {
            self.app.add_plugins(LibPlugin::new(path)?);
        }else if path.is_dir() {
            for entry in path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    self.app.add_plugins(LibPlugin::new(entry.path())?);
                }
            }
        }
        Ok(self)
    }

    pub fn insert_resource(&mut self, resource: impl Resource) -> &mut Self {
        self.app.insert_resource(resource);
        self
    }

    pub fn add_event<T>(&mut self) -> &mut Self
    where
        T: Event,
    {
        self.app.add_event::<T>();
        self
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.app.add_systems(schedule, systems);
        self
    }

    pub fn run<T: OneBotEventsEnumTrait + Debug + Send + Sync + 'static>(&mut self) -> AppExit {
        T::add_events(&mut self.app);
        self.app
            .add_systems(Startup, |world: &mut World| {
                KiraAsyncManager::global().insert("recv_event");
                rust_i18n::set_locale("zh-CN");
                let async_world = AsyncWorld::from_world(world);
                kira_async!("recv_event" => spawn_recv_loop::<T>(async_world).await);
            })
            .add_plugins(ScheduleRunnerPlugin::default())
            .add_plugins(AsyncEcsPlugin)
            .add_plugins(KiraAsyncManagerPlugin)
            .run()
    }
}
