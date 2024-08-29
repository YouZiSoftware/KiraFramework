use std::collections::HashMap;
use bevy_app::{App, Plugin};
use tokio::runtime::{EnterGuard, Runtime};

static mut ASYNC_MANAGER: Option<KiraAsyncManager> = None;

pub struct KiraAsyncManager(HashMap<String, KiraAsyncManagerInner>);

impl KiraAsyncManager {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn global() -> &'static Self {
        unsafe { ASYNC_MANAGER.as_ref().unwrap() }
    }

    pub fn insert(&self, key: &str) {
        unsafe { ASYNC_MANAGER.as_mut().unwrap().0.insert(key.to_string(), KiraAsyncManagerInner::new()) };
    }

    pub fn enter(&self, key: &str) -> Option<EnterGuard<'_>> {
        Some(self.0.get(key)?.runtime.enter())
    }
    pub fn runtime(&self, key: &str) -> Option<&Runtime> {
        Some(&self.0.get(key)?.runtime)
    }
}

pub struct KiraAsyncManagerInner {
    runtime: Runtime
}

impl KiraAsyncManagerInner {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new().unwrap(),
        }
    }
}

pub struct KiraAsyncManagerPlugin;

impl Plugin for KiraAsyncManagerPlugin {
    fn build(&self, app: &mut App) {
        unsafe { ASYNC_MANAGER = Some(KiraAsyncManager::new()) };
    }
}