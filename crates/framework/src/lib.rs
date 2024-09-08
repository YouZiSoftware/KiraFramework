pub mod configs;
pub mod network;
pub mod utils;
pub mod recv;
pub mod async_manager;
pub mod macros;
pub mod pretty_debug;
pub mod persistent;
pub mod lib_plugin;

use std::fmt::Debug;
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
use crate::network::connect::OneBotConnect;
use crate::network::events::OneBotEventsEnumTrait;
use crate::pretty_debug::KiraPrettyDebugToggle;
use crate::recv::spawn_recv_loop;

i18n!("locales");

pub struct BotApp {
    app: App,
    set_locale: bool,
    pretty_debug: bool,
}

impl BotApp {
    pub fn new() -> Self {
        let mut app = App::new();
        BotApp {
            app,
            set_locale: false,
            pretty_debug: true,
        }
    }

    pub fn bot_configs(&mut self, configs: BotConfigs) -> &mut Self {
        self.app.insert_resource(configs);
        self
    }

    pub fn onebot_connect(&mut self, mut connect: OneBotConnect) -> &mut Self {
        connect.set_world(AsyncWorld::from_world(self.app.world_mut()));
        self.app.insert_resource(connect);
        self
    }

    pub fn add_plugins<T>(&mut self, plugin: impl Plugins<T>) -> &mut Self {
        self.app.add_plugins(plugin);
        self
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

    pub fn set_locale(&mut self, locale: &str) -> &mut Self {
        rust_i18n::set_locale(locale);
        self.set_locale = true;
        self
    }

    pub fn pretty_debug(&mut self, pretty_debug: bool) -> &mut Self {
        self.pretty_debug = pretty_debug;
        self
    }

    pub fn run<T: OneBotEventsEnumTrait + Debug + Send + Sync + 'static>(&mut self) -> AppExit {
        T::add_events(&mut self.app);
        if !self.set_locale {
            self.set_locale("en-US");
        }
        self.app
            .insert_resource(KiraPrettyDebugToggle(self.pretty_debug))
            .add_systems(Startup, |world: &mut World| {
                KiraAsyncManager::global().insert("recv_event");
                let async_world = AsyncWorld::from_world(world);
                kira_async!("recv_event" => spawn_recv_loop::<T>(async_world).await);
            })
            .add_plugins(ScheduleRunnerPlugin::default())
            .add_plugins(AsyncEcsPlugin)
            .add_plugins(KiraAsyncManagerPlugin)
            .run()
    }
}
