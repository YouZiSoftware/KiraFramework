pub mod configs;
pub mod network;
pub mod utils;
pub mod recv;
pub mod macros;
pub mod pretty_debug;
pub mod persistent;
pub mod lib_plugin;

use std::fmt::Debug;
use rust_i18n::i18n;
use ur_ecs::app::{App, AppExit};
use ur_ecs::app::plugin::Plugins;
use ur_ecs::app::schedule_runner::ScheduleRunnerPlugin;
use ur_ecs::event::Event;
use ur_ecs::resource::Resource;
use ur_ecs::schedule::{ScheduleLabel, Startup};
use ur_ecs::system::IntoSystem;
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
        BotApp {
            app: App::new(),
            set_locale: false,
            pretty_debug: true,
        }
    }

    pub fn bot_configs(&mut self, configs: BotConfigs) -> &mut Self {
        self.app.insert_resource(configs);
        self
    }

    pub fn onebot_connect(&mut self, mut connect: OneBotConnect) -> &mut Self {
        connect.set_world(self.app.world().clone());
        self.app.insert_resource(connect);
        self
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        self.app.add_plugins(plugins);
        self
    }

    pub fn insert_resource<R: Resource + 'static + Send + Sync>(&mut self, resource: R) -> &mut Self {
        self.app.insert_resource(resource);
        self
    }

    pub fn add_event<T: Event + 'static + Send + Sync>(&mut self) -> &mut Self {
        self.app.add_event::<T>();
        self
    }

    pub fn add_systems<S, Marker>(&mut self, schedule: impl ScheduleLabel, systems: S) -> &mut Self
    where
        S: IntoSystem<S, Marker> + 'static,
        Marker: 'static,
    {
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
            .add_systems(Startup, spawn_recv_loop::<T>)
            .add_plugins(ScheduleRunnerPlugin::default())
            .run()
    }
}